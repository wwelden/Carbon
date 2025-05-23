module Evaluator where
    import qualified Parser as P
    import Parser (Statement(..), Expr(..), BOp(..), ClassMember(..), Var, ClassName, FieldName, MethodName, Pattern(..), Literal(..), MatchCase)
    import Debug.Trace
    import Data.Maybe
    import System.IO.Unsafe
    import System.IO (hFlush, stdout)

    data Value = IntVal Int
               | DoubleVal Double
               | BoolVal Bool
               | StringVal String
               | NullVal
               | ArrayVal [Value]
               | FuncVal Env Var Expr
               | TypedFuncVal Env [(P.Type, Var)] P.Type [Statement] Expr
               | ObjectVal ClassName [(FieldName, Value)] [(MethodName, Var, Expr)]
               | PrintVal String
               deriving (Show, Eq)

    data Type = IntType | BoolType | StringType | ArrayType Type
        deriving (Show, Eq)

    type Loc = Int
    type Env = [(Var, Value)]
    type Store = [(Loc, Value)]
    type ClassEnv = [(ClassName, ClassDef)]
    data ClassDef = ClassDef [FieldName] [(MethodName, Var, Expr)]
                    deriving (Show, Eq)
    data Context = Context {env :: Env, store :: Store, classes :: ClassEnv} deriving (Eq, Show)

    lookupEnv :: Var -> Env -> Maybe Value
    lookupEnv = lookup

    updateEnv :: Var -> Value -> Env -> Env
    updateEnv x v env = filter (\(key, _) -> key /= x) env ++ [(x, v)]

    lookupClass :: ClassName -> ClassEnv -> Maybe ClassDef
    lookupClass = lookup

    updateClasses :: ClassName -> ClassDef -> ClassEnv -> ClassEnv
    updateClasses cn cd cenv = filter (\(key, _) -> key /= cn) cenv ++ [(cn, cd)]

    lookupField :: FieldName -> [(FieldName, Value)] -> Maybe Value
    lookupField = lookup

    updateField :: FieldName -> Value -> [(FieldName, Value)] -> [(FieldName, Value)]
    updateField fn v fields = filter (\(key, _) -> key /= fn) fields ++ [(fn, v)]

    lookupMethod :: MethodName -> [(MethodName, Var, Expr)] -> Maybe (Var, Expr)
    lookupMethod mn methods = case lookup mn [(mn', (var, expr)) | (mn', var, expr) <- methods] of
                                Just (var, expr) -> Just (var, expr)
                                Nothing -> Nothing

    realOfValue :: Value -> Maybe Double
    realOfValue (IntVal x) = Just $ fromIntegral x
    realOfValue (DoubleVal f) = Just f
    realOfValue (BoolVal b) = Nothing

    binaryOp :: (Int -> Int -> Int) -> (Double -> Double -> Double) -> Value -> Value -> Maybe Value
    binaryOp f g (IntVal i1) (IntVal i2) = Just $ IntVal (f i1 i2)
    binaryOp f g (DoubleVal f1) (DoubleVal f2) = Just $ DoubleVal (g f1 f2)
    binaryOp f1 g (IntVal i) (DoubleVal f) = Just $ DoubleVal (g (fromIntegral i) f)
    binaryOp f1 g (DoubleVal f) (IntVal i) = Just $ DoubleVal (g f (fromIntegral i))
    binaryOp _ _ _ _ = Nothing

    divOp, expOp, modOp :: Value -> Value -> Maybe Value
    divOp (IntVal i1) (IntVal i2) = Just $ IntVal (i1 `div` i2)
    divOp (DoubleVal f1) (DoubleVal f2) = Just $ DoubleVal (f1 / f2)
    divOp (IntVal i) (DoubleVal f) = Just $ DoubleVal (fromIntegral i / f)
    divOp (DoubleVal f) (IntVal i) = Just $ DoubleVal (f / fromIntegral i)
    divOp _ _ = Nothing

    expOp (IntVal i1) (IntVal i2) = Just $ IntVal (i1 ^ i2)
    expOp (DoubleVal f1) (DoubleVal f2) = Just $ DoubleVal (f1 ** f2)
    expOp (IntVal i) (DoubleVal f) = Just $ DoubleVal (fromIntegral i ** f)
    expOp (DoubleVal f) (IntVal i) = Just $ DoubleVal (f ** fromIntegral i)
    expOp _ _ = Nothing

    modOp (IntVal i1) (IntVal i2) = if i2 == 0 then Nothing else Just $ IntVal (i1 `mod` i2)
    modOp _ _ = Nothing

    sqrtOp, negateOp :: Value -> Maybe Value
    sqrtOp (IntVal i) = Just $ DoubleVal (sqrt (fromIntegral i))
    sqrtOp (DoubleVal f) = Just $ DoubleVal (sqrt f)
    sqrtOp _ = Nothing

    negateOp (IntVal i) = Just $ IntVal (-i)
    negateOp (DoubleVal f) = Just $ DoubleVal (-f)
    negateOp _ = Nothing

    eqOp, neqOp, gtOp, ltOp, geqOp, leqOp :: Value -> Value -> Maybe Value
    eqOp (IntVal i1) (IntVal i2) = Just $ BoolVal $ i1 == i2
    eqOp (DoubleVal f1) (DoubleVal f2) = Just $ BoolVal $ f1 == f2
    eqOp (IntVal i) (DoubleVal f) = Just $ BoolVal $ fromIntegral i == f
    eqOp (DoubleVal f) (IntVal i) = Just $ BoolVal $ f == fromIntegral i
    eqOp (BoolVal b1) (BoolVal b2) = Just $ BoolVal $ b1 == b2
    eqOp (StringVal s1) (StringVal s2) = Just $ BoolVal $ s1 == s2
    eqOp (NullVal) (NullVal) = Just $ BoolVal True
    eqOp (ArrayVal v1) (ArrayVal v2) = Just $ BoolVal $ length v1 == length v2 && and (zipWith (\a b -> eqOp a b == Just (BoolVal True)) v1 v2)
    eqOp (FuncVal _ _ _) (FuncVal _ _ _) = Just $ BoolVal False
    eqOp _ _ = Just $ BoolVal False

    neqOp v1 v2 = case eqOp v1 v2 of
        Just (BoolVal b) -> Just $ BoolVal (not b)
        _ -> Nothing

    gtOp (IntVal i1) (IntVal i2) = Just $ BoolVal $ i1 > i2
    gtOp (DoubleVal f1) (DoubleVal f2) = Just $ BoolVal $ f1 > f2
    gtOp (IntVal i) (DoubleVal f) = Just $ BoolVal $ fromIntegral i > f
    gtOp (DoubleVal f) (IntVal i) = Just $ BoolVal $ f > fromIntegral i
    gtOp _ _ = Nothing

    ltOp (IntVal i1) (IntVal i2) = Just $ BoolVal $ i1 < i2
    ltOp (DoubleVal f1) (DoubleVal f2) = Just $ BoolVal $ f1 < f2
    ltOp (IntVal i) (DoubleVal f) = Just $ BoolVal $ fromIntegral i < f
    ltOp (DoubleVal f) (IntVal i) = Just $ BoolVal $ f < fromIntegral i
    ltOp _ _ = Nothing

    geqOp (IntVal i1) (IntVal i2) = Just $ BoolVal $ i1 >= i2
    geqOp (DoubleVal f1) (DoubleVal f2) = Just $ BoolVal $ f1 >= f2
    geqOp (IntVal i) (DoubleVal f) = Just $ BoolVal $ fromIntegral i >= f
    geqOp (DoubleVal f) (IntVal i) = Just $ BoolVal $ f >= fromIntegral i
    geqOp _ _ = Nothing

    leqOp (IntVal i1) (IntVal i2) = Just $ BoolVal $ i1 <= i2
    leqOp (DoubleVal f1) (DoubleVal f2) = Just $ BoolVal $ f1 <= f2
    leqOp (IntVal i) (DoubleVal f) = Just $ BoolVal $ fromIntegral i <= f
    leqOp (DoubleVal f) (IntVal i) = Just $ BoolVal $ f <= fromIntegral i
    leqOp _ _ = Nothing

    evalStmt :: Context -> Statement -> Maybe (Context, Value)
    evalStmt ctx (ExprStmt e) =
        do  (val, st) <- eval ctx e
            Just (ctx{store = st}, val)
    evalStmt ctx (LetStmt var e) =
        do  (val, st) <- eval ctx e
            Just (ctx{store = st, env = updateEnv var val (env ctx)}, NullVal)
    evalStmt ctx (ConstStmt var e) =
        let defaultValue = (NullVal, store ctx)
            recEnv :: Env
            recEnv = updateEnv var lazyVal (env ctx)
            (lazyVal, st) = fromMaybe defaultValue (eval ctx {env = recEnv} e)
        in Just (ctx{store = st, env = recEnv}, NullVal)
    evalStmt ctx (ClassStmt className members) =
        let fields = [fname | FieldDecl fname <- members]
            methods = [(mname, var, expr) | MethodDecl mname var expr <- members]
            classDef = ClassDef fields methods
            newClasses = updateClasses className classDef (classes ctx)
        in Just (ctx{classes = newClasses}, NullVal)
    evalStmt ctx (TypedVarStmt varType var e) =
        do  (val, st) <- eval ctx e
            Just (ctx{store = st, env = updateEnv var val (env ctx)}, NullVal)
    evalStmt ctx (FnDeclStmt name params retType stmts retExpr) =
        let func = TypedFuncVal (env ctx) params retType stmts retExpr
        in Just (ctx{env = updateEnv name func (env ctx)}, NullVal)
    evalStmt ctx (ForInStmt var arrayExpr stmts) =
        case eval ctx arrayExpr of
            Just (ArrayVal vals, st) -> forInStmtLoop ctx{store = st} var vals stmts
            _ -> Nothing

    -- Pattern matching functions
    patternMatch :: Pattern -> Value -> Maybe [(Var, Value)]
    patternMatch (LitPat lit) val =
        case (lit, val) of
            (IntLit i, IntVal v) -> if i == v then Just [] else Nothing
            (RealLit r, DoubleVal v) -> if r == v then Just [] else Nothing
            (RealLit r, IntVal v) -> if r == fromIntegral v then Just [] else Nothing
            (IntLit i, DoubleVal v) -> if fromIntegral i == v then Just [] else Nothing
            (BoolLit b, BoolVal v) -> if b == v then Just [] else Nothing
            (StringLit s, StringVal v) -> if s == v then Just [] else Nothing
            (NullLit, NullVal) -> Just []
            _ -> Nothing
    patternMatch (VarPat var) val = Just [(var, val)]
    patternMatch WildcardPat _ = Just []
    patternMatch (ArrayPat patterns) (ArrayVal values) =
        if length patterns == length values
        then foldl combineBindings (Just []) (zipWith patternMatch patterns values)
        else Nothing
    patternMatch (ArrayPat _) _ = Nothing

    combineBindings :: Maybe [(Var, Value)] -> Maybe [(Var, Value)] -> Maybe [(Var, Value)]
    combineBindings Nothing _ = Nothing
    combineBindings _ Nothing = Nothing
    combineBindings (Just bindings1) (Just bindings2) = Just (bindings1 ++ bindings2)

    tryMatchCases :: Context -> Value -> [MatchCase] -> Maybe (Value, Store)
    tryMatchCases ctx _ [] = Nothing
    tryMatchCases ctx val ((pattern, expr) : cases) =
        case patternMatch pattern val of
            Just bindings ->
                let newEnv = foldl (\env (var, value) -> updateEnv var value env) (env ctx) bindings
                in eval ctx{env = newEnv} expr
            Nothing -> tryMatchCases ctx val cases

    eval :: Context -> Expr -> Maybe (Value, Store)
    eval ctx (IntExpr i) = Just (IntVal i, store ctx)
    eval ctx (RealExpr f) = Just (DoubleVal f, store ctx)
    eval ctx EExpr = Just (DoubleVal (exp 1), store ctx)
    eval ctx PhiExpr = Just (DoubleVal ((1 + sqrt 5) / 2), store ctx)
    eval ctx PiExpr = Just (DoubleVal pi, store ctx)
    eval ctx AvogadroExpr = Just (DoubleVal 6.02214076e23, store ctx)
    eval ctx (NegExpr e) = threadStore negateOp ctx e
    eval ctx (SqrtExpr e) = threadStore sqrtOp ctx e
    eval ctx (IfExpr e1 e2 e3) =
        case eval ctx e1 of
            Just (BoolVal x, st) -> eval ctx{store = st} $ if x then e2 else e3
            _ -> Nothing
    eval ctx (TernaryExpr e1 e2 e3) =
        case eval ctx e1 of
            Just (BoolVal x, st) -> eval ctx{store = st} $ if x then e2 else e3
            _ -> Nothing
    eval ctx (BOpExpr PlusOp e1 e2) = threadStore2 (binaryOp (+) (+)) ctx e1 e2
    eval ctx (BOpExpr MinusOp e1 e2) = threadStore2 (binaryOp (-) (-)) ctx e1 e2
    eval ctx (BOpExpr MultOp e1 e2) = threadStore2 (binaryOp (*) (*)) ctx e1 e2
    eval ctx (BOpExpr DivOp e1 e2) = threadStore2 divOp ctx e1 e2
    eval ctx (BOpExpr ModOp e1 e2) = threadStore2 modOp ctx e1 e2
    eval ctx (BOpExpr ExpOp e1 e2) = threadStore2 expOp ctx e1 e2
    eval ctx (BOpExpr EqOp e1 e2) = threadStore2 eqOp ctx e1 e2
    eval ctx (BOpExpr NeqOp e1 e2) = threadStore2 neqOp ctx e1 e2
    eval ctx (BOpExpr GtOp e1 e2) = threadStore2 gtOp ctx e1 e2
    eval ctx (BOpExpr LtOp e1 e2) = threadStore2 ltOp ctx e1 e2
    eval ctx (BOpExpr GeqOp e1 e2) = threadStore2 geqOp ctx e1 e2
    eval ctx (BOpExpr LeqOp e1 e2) = threadStore2 leqOp ctx e1 e2
    eval ctx (BOpExpr AndOp e1 e2) =
        case eval ctx e1 of
            r@(Just (BoolVal False, st)) -> r
            Just (BoolVal True, st) ->
                case eval ctx{store = st} e2 of
                    r@(Just (BoolVal x, st2)) -> r
                    _ -> Nothing
            _ -> Nothing
    eval ctx (BOpExpr OrOp e1 e2) =
        case eval ctx e1 of
            r@(Just (BoolVal True, st)) -> r
            Just (BoolVal False, st) ->
                case eval ctx{store = st} e2 of
                    r@(Just (BoolVal x, st2)) -> r
                    _ -> Nothing
            _ -> Nothing
    eval ctx (BoolExpr b) = Just (BoolVal b, store ctx)
    eval ctx NullExpr = Just (NullVal, store ctx)
    eval ctx (StringExpr s) = Just (StringVal s, store ctx)
    eval ctx (VarExpr x) =
        do  val <- lookupEnv x (env ctx)
            Just (val, store ctx)
    eval ctx (ForExpr var init cond update body) =
        do  (initVal, st) <- eval ctx init
            let newEnv = updateEnv var initVal (env ctx)
            forLoop ctx{store = st, env = newEnv} var cond update body
    eval ctx (ToStringExpr e) =
        case eval ctx e of
            Just (val, st) -> Just (StringVal (showVal val), st)
            _ -> Nothing
    eval ctx (TypeOfExpr e) =
        case eval ctx e of
            Just (IntVal _, st) -> Just (StringVal "number", st)
            Just (DoubleVal _, st) -> Just (StringVal "number", st)
            Just (BoolVal _, st) -> Just (StringVal "boolean", st)
            Just (StringVal _, st) -> Just (StringVal "string", st)
            Just (NullVal, st) -> Just (StringVal "null", st)
            Just (ArrayVal _, st) -> Just (StringVal "array", st)
            Just (FuncVal _ _ _, st) -> Just (StringVal "function", st)
            Just (ObjectVal _ _ _, st) -> Just (StringVal "object", st)
            _ -> Nothing
    eval ctx (ArrayExpr exprs) =
        do  (vals, st) <- evalExprList ctx exprs
            Just (ArrayVal vals, st)
    eval ctx (NotExpr e) =
        case eval ctx e of
            Just (BoolVal b, st) -> Just (BoolVal (not b), st)
            _ -> Nothing
    eval ctx (FuncExpr var e) = Just (FuncVal (env ctx) var e, store ctx)
    eval ctx (FunctionExpr var e) = Just (FuncVal (env ctx) var e, store ctx)
    eval ctx (ApplyExpr func e) =
        do  (funcVal, st) <- eval ctx func
            (argVal, st2) <- eval ctx{store = st} e
            case funcVal of
                FuncVal ctx2 var body -> eval ctx{store = st2, env = updateEnv var argVal ctx2} body
                _ -> Nothing
    eval ctx (AssignExpr e1 e2) =
        case eval ctx e1 of
            Just (FuncVal ctx2 var body, st) ->
                do  (val, st2) <- eval ctx{store = st} e2
                    Just (val, st2)
            Just (ArrayVal vals, st) ->
                case eval ctx{store = st} e2 of
                    Just (val, st2) -> Just (val, st2)
                    _ -> Nothing
            _ -> Nothing
    eval ctx (SeqExpr e1 e2) =
        case eval ctx e1 of
            Just (_, st) -> eval ctx{store = st} e2
            _ -> Nothing
    eval ctx (RefExpr e) =
        case eval ctx e of
            Just (val, st) -> Just (IntVal (length (store ctx)), (length (store ctx), val):st)
            _ -> Nothing
    eval ctx (DerefExpr e) =
        case eval ctx e of
            Just (IntVal loc, st) ->
                case lookup loc st of
                    Just val -> Just (val, st)
                    Nothing -> Nothing
            _ -> Nothing
    eval ctx (WhileExpr e1 e2) =
        case eval ctx e1 of
            Just (BoolVal True, st) ->
                case eval ctx{store = st} e2 of
                    Just (val, st2) -> eval ctx{store = st2} (WhileExpr e1 e2)
                    _ -> Nothing
            Just (BoolVal False, st) -> Just (NullVal, st)
            _ -> Nothing
    eval ctx (NewExpr className args) =
        case lookupClass className (classes ctx) of
            Just (ClassDef fields methods) ->
                do  (argVals, st) <- evalExprList ctx args
                    let fieldValues = zip fields argVals
                    Just (ObjectVal className fieldValues methods, st)
            Nothing -> Nothing
    eval ctx (MethodCallExpr objExpr methodName argExpr) =
        do  (objVal, st) <- eval ctx objExpr
            (argVal, st2) <- eval ctx{store = st} argExpr
            case objVal of
                ObjectVal _ _ methods ->
                    case lookupMethod methodName methods of
                        Just (param, body) ->
                            let newEnv = updateEnv param argVal (updateEnv "this" objVal (env ctx))
                            in eval ctx{store = st2, env = newEnv} body
                        Nothing -> Nothing
                _ -> Nothing
    eval ctx (FieldAccessExpr objExpr fieldName) =
        do  (objVal, st) <- eval ctx objExpr
            case objVal of
                ObjectVal _ fields _ ->
                    case lookupField fieldName fields of
                        Just val -> Just (val, st)
                        Nothing -> Nothing
                _ -> Nothing
    eval ctx ThisExpr =
        do  val <- lookupEnv "this" (env ctx)
            Just (val, store ctx)
    eval ctx (ArrayLenExpr e) =
        case eval ctx e of
            Just (ArrayVal vals, st) -> Just (IntVal (length vals), st)
            _ -> Nothing
    eval ctx (PrintExpr e) =
        case eval ctx e of
            Just (val, st) ->
                let output = showVal val
                in Just (PrintVal output, st)
            _ -> Nothing
    eval ctx (ForInExpr var arrayExpr body) =
        case eval ctx arrayExpr of
            Just (ArrayVal vals, st) -> forInLoop ctx{store = st} var vals body
            _ -> Nothing
    eval ctx (ForCStyleExpr var initExpr condExpr updateExpr body) =
        do  (initVal, st) <- eval ctx initExpr
            let newEnv = updateEnv var initVal (env ctx)
            forCStyleLoop ctx{store = st, env = newEnv} var condExpr updateExpr body
    eval ctx (ForWhileExpr condExpr body) =
        forWhileLoop ctx condExpr body
    eval ctx (MatchExpr expr cases) =
        case eval ctx expr of
            Just (val, st) ->
                case tryMatchCases ctx{store = st} val cases of
                    Just result -> Just result
                    Nothing -> Nothing  -- No pattern matched
            Nothing -> Nothing

    forLoop :: Context -> Var -> Expr -> Expr -> Expr -> Maybe (Value, Store)
    forLoop ctx var cond update body =
        case eval ctx cond of
            Just (BoolVal True, st) ->
                case eval ctx{store = st} body of
                    Just (_, st2) ->
                        case eval ctx{store = st2} update of
                            Just (updateVal, st3) ->
                                let newEnv = updateEnv var updateVal (env ctx)
                                in forLoop ctx{store = st3, env = newEnv} var cond update body
                            _ -> Nothing
                    _ -> Nothing
            Just (BoolVal False, st) -> Just (NullVal, st)
            _ -> Nothing

    evalExprList :: Context -> [Expr] -> Maybe ([Value], Store)
    evalExprList ctx [] = Just ([], store ctx)
    evalExprList ctx (e:es) =
        do  (val, st) <- eval ctx e
            (vals, st2) <- evalExprList ctx{store = st} es
            Just (val:vals, st2)

    showVal :: Value -> String
    showVal (IntVal i) = show i
    showVal (DoubleVal f) = show f
    showVal (BoolVal b) = if b then "true" else "false"
    showVal (StringVal s) = s
    showVal NullVal = "null"
    showVal (ArrayVal vals) = "[" ++ (concatMap (\v -> showVal v ++ ", ") vals) ++ "]"
    showVal (FuncVal _ var _) = "function " ++ var
    showVal (TypedFuncVal _ _ _ _ _) = "function"
    showVal (ObjectVal className _ _) = "object " ++ className
    showVal (PrintVal s) = s

    threadStore :: (Value -> Maybe Value) -> Context -> Expr -> Maybe (Value, Store)
    threadStore f ctx a =
        do (av, st) <- eval ctx a
           fr <- f av
           Just (fr, st)

    threadStore2 :: (Value -> Value -> Maybe Value) -> Context -> Expr -> Expr -> Maybe (Value, Store)
    threadStore2 f ctx a b =
        do (av, st) <- eval ctx a
           (bv, st2) <- eval ctx{store = st} b
           fr <- f av bv
           Just (fr, st2)

    forInLoop :: Context -> Var -> [Value] -> Expr -> Maybe (Value, Store)
    forInLoop ctx var [] body = Just (NullVal, store ctx)
    forInLoop ctx var (v:vs) body =
        let newEnv = updateEnv var v (env ctx)
        in case eval ctx{env = newEnv} body of
            Just (_, st) -> forInLoop ctx{store = st} var vs body
            Nothing -> Nothing

    forCStyleLoop :: Context -> Var -> Expr -> Expr -> Expr -> Maybe (Value, Store)
    forCStyleLoop ctx var condExpr updateExpr body =
        case eval ctx condExpr of
            Just (BoolVal True, st) ->
                case eval ctx{store = st} body of
                    Just (_, st2) ->
                        case eval ctx{store = st2} updateExpr of
                            Just (updateVal, st3) ->
                                let newEnv = updateEnv var updateVal (env ctx)
                                in forCStyleLoop ctx{store = st3, env = newEnv} var condExpr updateExpr body
                            _ -> Nothing
                    _ -> Nothing
            Just (BoolVal False, st) -> Just (NullVal, st)
            _ -> Nothing

    forWhileLoop :: Context -> Expr -> Expr -> Maybe (Value, Store)
    forWhileLoop ctx condExpr body =
        case eval ctx condExpr of
            Just (BoolVal True, st) ->
                case eval ctx{store = st} body of
                    Just (_, st2) -> forWhileLoop ctx{store = st2} condExpr body
                    _ -> Nothing
            Just (BoolVal False, st) -> Just (NullVal, st)
            _ -> Nothing

    forInStmtLoop :: Context -> Var -> [Value] -> [Statement] -> Maybe (Context, Value)
    forInStmtLoop ctx var [] stmts = Just (ctx, NullVal)
    forInStmtLoop ctx var (v:vs) stmts =
        let newEnv = updateEnv var v (env ctx)
            newCtx = ctx{env = newEnv}
        in case evalStmtList newCtx stmts of
            Just (ctx2, _) -> forInStmtLoop ctx2 var vs stmts
            Nothing -> Nothing

    evalStmtList :: Context -> [Statement] -> Maybe (Context, Value)
    evalStmtList ctx [] = Just (ctx, NullVal)
    evalStmtList ctx (s:ss) =
        case evalStmt ctx s of
            Just (newCtx, _) -> evalStmtList newCtx ss
            Nothing -> Nothing
