{
module Parser where
import Lexer
import Data.Maybe
}

%name parse
%tokentype { Token }
%error { parseError }
%monad { Maybe }

%token
    int             { IntTok $$ }
    real            { RealTok $$ }
    E               { ETok }
    PI              { PiTok }
    PHI             { PhiTok }
    AVOGADRO        { AvogadroTok }
    '+'             { PlusTok }
    '-'             { MinusTok }
    '*'             { MultTok }
    '/'             { DivTok }
    '+='            { PlusEqTok }
    '-='            { MinusEqTok }
    '*='            { MultEqTok }
    '/='            { DivEqTok }
    '%='            { ModEqTok }
    '--'            { DecrementTok }
    '('             { LParenTok }
    ')'             { RParenTok }
    '!'             { NotTok }
    '%'             { ModTok }
    '=='            { EqTok }
    '**'            { ExpTok }
    '>'             { GTTok }
    '<'             { LTTok }
    '<='            { LeqTok }
    '>='            { GeqTok }
    '!='            { NeqTok }
    'Math.sqrt'     { MathSqrtTok }
    -- Boolean and control tokens
    true            { TrueTok }
    false           { FalseTok }
    null            { NullTok }
    var             { VarTok $$ }
    string          { StringTok $$ }
    -- Control flow
    if              { IfTok }
    else            { ElseTok }
    let             { LetTok }
    const           { ConstTok }
    for             { ForTok }
    '='             { AssignTok }
    -- Logic operators
    '&&'            { AndTok }
    '||'            { OrTok }
    ';'             { SemicolonTok }

    fn              { FnTok}
    function        { FunctionTok}
    '=>'            { ArrowTok}
    toString        { ToStringTok}
    typeof          { TypeOfTok}
    print           { PrintTok}
    intType         { IntTypeTok}
    boolType        { BoolTypeTok}
    stringType      { StringTypeTok}
    in              { InTok}
    len             { LenTok}
    ','             { CommaTok}
    return          { ReturnTok}
    '&'             { RefTok }
    while           { WhileTok }
    match           { MatchTok }
    '|'             { PipeTok }
    '_'             { WildcardTok }

    cname           { CnameTok $$}
    class           { ClassTok }
    new             { NewTok }
    this            { ThisTok }
    '.'             { DotTok }
    '{'             { LBraceTok }
    '}'             { RBraceTok }
    '['             { LBracketTok }
    ']'             { RBracketTok }
    '?'             { QuestionTok }
    ':'             { ColonTok }
    '++'            { IncrementTok }
    err             { ErrTok }
    isErr           { IsErrTok }

-- Proper precedence and associativity (lowest to highest)
%nonassoc ';'
%left '|'
%right '=>'
%right '?' ':'
%left '||'
%left '&&'
%left '==' '!='
%left '>' '<' '<=' '>='
%left '+' '-'
%left '*' '/' '%'
%right '**'
%right '!' 'UMINUS'
%left '(' ')' '[' ']' '.'
%right '='
%nonassoc if else let const match
%nonassoc for while return function fn in

%%
Statement: Expr ';'                   {ExprStmt $1}
    | let var '=' Expr ';'            {LetStmt $2 $4}
    | const var '=' Expr ';'          {ConstStmt $2 $4}
    | Type var '=' Expr ';'           {TypedVarStmt $1 $2 $4}
    | fn var '(' ParamList ')' '{' StmtList '}' {FnDeclStmt $2 $4 Nothing $7 (VarExpr "null")}
    | fn var '(' ParamList ')' '=>' Type '{' StmtList '}' {FnDeclStmt $2 $4 (Just $7) $9 (VarExpr "null")}
    | class cname '{' ClassMembers '}' {ClassStmt $2 $4}
    | for var in Expr '{' StmtList '}' {ForInStmt $2 $4 $6}
    | for '(' Expr ')' '{' StmtList '}' {WhileStmt $3 $6}
    | var '+=' Expr ';'               {CompoundAssignStmt $1 PlusEq $3}
    | var '-=' Expr ';'               {CompoundAssignStmt $1 MinusEq $3}
    | var '*=' Expr ';'               {CompoundAssignStmt $1 MultEq $3}
    | var '/=' Expr ';'               {CompoundAssignStmt $1 DivEq $3}
    | var '%=' Expr ';'               {CompoundAssignStmt $1 ModEq $3}
    | var '++' ';'                    {IncrementStmt $1}
    | var '--' ';'                    {DecrementStmt $1}
    | return Expr ';'                 {ReturnStmt $2}

ExprList : {- empty -}                { [] }
    | Expr                            {[$1]}
    | Expr ',' ExprList               { $1 : $3 }

Type : intType                        { IntType }
    | boolType                        { BoolType }
    | stringType                      { StringType }
    | Type '[' ']'                    { ArrayType $1 }

ParamList : {- empty -}               { [] }
    | Param                           { [$1] }
    | Param ',' ParamList             { $1 : $3 }

Param : Type var                      { ($1, $2) }

StmtList : {- empty -}                { [] }
    | Statement StmtList              { $1 : $2 }

ClassMembers : {- empty -}            { [] }
    | ClassMember ClassMembers        { $1 : $2 }

ClassMember : var ';'                 { FieldDecl $1 }
    | var '(' var ')' '{' return Expr '}' { MethodDecl $1 $3 $7 }

Expr: int                         { IntExpr $1 }
    | real                        { RealExpr $1 }
    | E                           { EExpr }
    | PHI                         { PhiExpr }
    | PI                          { PiExpr }
    | AVOGADRO                    { AvogadroExpr }
    | true                        { BoolExpr True}
    | false                       { BoolExpr False}
    | null                        { NullExpr }
    | string                      { StringExpr $1 }
    | var                         { VarExpr $1 }
    | '(' Expr ')'                { $2 }
    | '!' Expr                    { NotExpr $2 }
    | 'Math.sqrt' '(' Expr ')'    { SqrtExpr $3 }
    | '-' Expr %prec 'UMINUS'     { NegExpr $2 }
    | Expr '+' Expr               { BOpExpr PlusOp $1 $3 }
    | Expr '-' Expr               { BOpExpr MinusOp $1 $3 }
    | Expr '*' Expr               { BOpExpr MultOp $1 $3 }
    | Expr '/' Expr               { BOpExpr DivOp $1 $3 }
    | Expr '**' Expr              { BOpExpr ExpOp $1 $3 }
    | Expr '%' Expr               { BOpExpr ModOp $1 $3 }
    | Expr '==' Expr              { BOpExpr EqOp $1 $3 }
    | Expr '!=' Expr              { BOpExpr NeqOp $1 $3 }
    | Expr '>' Expr               { BOpExpr GtOp $1 $3 }
    | Expr '<' Expr               { BOpExpr LtOp $1 $3 }
    | Expr '&&' Expr              { BOpExpr AndOp $1 $3 }
    | Expr '||' Expr              { BOpExpr OrOp $1 $3 }
    | Expr '<=' Expr              { BOpExpr LeqOp $1 $3 }
    | Expr '>=' Expr              { BOpExpr GeqOp $1 $3 }
    | if Expr '{' Expr '}' else '{' Expr '}' { IfExpr $2 $4 $8 }
    | Expr '?' Expr ':' Expr      { TernaryExpr $1 $3 $5 }
    | for var in Expr '{' Expr '}'    { ForInExpr $2 $4 $6 }
    | for '(' Expr ')' '{' Expr '}'   { WhileExpr $3 $6 }
    | toString '(' Expr ')'       { ToStringExpr $3 }
    | typeof '(' Expr ')'         { TypeOfExpr $3 }
    | '[' ExprList ']'            { ArrayExpr $2}
    | var '=>' Expr               { FuncExpr $1 $3 }
    | function '(' var ')' '{' return Expr '}' { FunctionExpr $3 $7 }
    | Expr '(' Expr ')'           { ApplyExpr $1 $3 }
    | while '(' Expr ')' '{' Expr '}' { WhileExpr $3 $6 }

    | new cname '(' ExprList ')'  { NewExpr $2 $4 }
    | Expr '.' var '(' Expr ')'   { MethodCallExpr $1 $3 $5 }
    | Expr '.' var                { FieldAccessExpr $1 $3 }
    | Expr '.' len                { ArrayLenExpr $1 }
    | print '(' Expr ')'          { PrintExpr $3 }
    | match Expr '{' MatchCases '}' { MatchExpr $2 $4 }
    | err '(' string ')'          { ErrorExpr $3 }
    | '(' Expr ',' ExprList ')'   { TupleExpr ($2 : $4) }
    | isErr '(' Expr ')'          { IsErrorExpr $3 }
    | this                        { ThisExpr }

MatchCases : MatchCase               { [$1] }
    | MatchCase ',' MatchCases       { $1 : $3 }

MatchCase : Pattern '=>' Expr        { ($1, $3) }

Pattern : int                        { LitPat (IntLit $1) }
    | real                           { LitPat (RealLit $1) }
    | true                           { LitPat (BoolLit True) }
    | false                          { LitPat (BoolLit False) }
    | string                         { LitPat (StringLit $1) }
    | null                           { LitPat NullLit }
    | var                            { VarPat $1 }
    | '_'                            { WildcardPat }
    | '[' PatternList ']'            { ArrayPat $2 }
    | '(' Pattern ')'                { $2 }

PatternList : {- empty -}            { [] }
    | Pattern                        { [$1] }
    | Pattern ',' PatternList        { $1 : $3 }

{

type Var = String
type ClassName = String
type FieldName = String
type MethodName = String
type MatchCase = (Pattern, Expr)

data ClassMember = FieldDecl FieldName
                 | MethodDecl MethodName Var Expr
                 deriving (Show, Eq)

data Statement = ExprStmt Expr
    | LetStmt Var Expr
    | ConstStmt Var Expr
    | TypedVarStmt Type Var Expr
    | FnDeclStmt Var [(Type, Var)] (Maybe Type) [Statement] Expr
    | ClassStmt ClassName [ClassMember]
    | ForInStmt Var Expr [Statement]
    | WhileStmt Expr [Statement]
    | CompoundAssignStmt Var CompoundOp Expr
    | IncrementStmt Var
    | DecrementStmt Var
    | ReturnStmt Expr
    deriving (Show, Eq)

data Type = IntType | BoolType | StringType | ArrayType Type
    deriving (Show, Eq)

data Pattern = LitPat Literal
    | VarPat Var
    | WildcardPat
    | ArrayPat [Pattern]
    deriving (Show, Eq)

data Literal = IntLit Int
    | RealLit Double
    | BoolLit Bool
    | StringLit String
    | NullLit
    deriving (Show, Eq)

data Expr = IntExpr Int
    | RealExpr Double
    | EExpr
    | PhiExpr
    | PiExpr
    | AvogadroExpr
    | NegExpr Expr
    | SqrtExpr Expr
    | IfExpr Expr Expr Expr
    | TernaryExpr Expr Expr Expr
    | NotExpr Expr
    | StringExpr String
    | VarExpr Var
    | BOpExpr BOp Expr Expr
    | BoolExpr Bool
    | NullExpr
    | ToStringExpr Expr
    | TypeOfExpr Expr
    | ArrayExpr [Expr]
    | FuncExpr Var Expr

    | FunctionExpr Var Expr
    | ApplyExpr Expr Expr
    | WhileExpr Expr Expr
    | NewExpr ClassName [Expr]
    | MethodCallExpr Expr MethodName Expr
    | FieldAccessExpr Expr FieldName
    | ArrayLenExpr Expr
    | PrintExpr Expr
    | ForInExpr Var Expr Expr
    | MatchExpr Expr [MatchCase]
    | ErrorExpr String
    | TupleExpr [Expr]
    | IsErrorExpr Expr
    | ThisExpr
    deriving (Show, Eq)

data BOp = PlusOp
    | MinusOp
    | MultOp
    | DivOp
    | ModOp
    | ExpOp
    | EqOp
    | NeqOp
    | GtOp
    | LtOp
    | LeqOp
    | GeqOp
    | AndOp
    | OrOp
    deriving (Show, Eq)

data CompoundOp = PlusEq | MinusEq | MultEq | DivEq | ModEq
    deriving (Show, Eq)

parseError :: [Token] -> Maybe a
parseError _ = Nothing

}
