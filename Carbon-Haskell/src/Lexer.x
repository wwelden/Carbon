{
module Lexer where
import Debug.Trace
}
%wrapper "basic"

$digit      = [0-9]                     -- Digit
$alpha      = [a-zA-Z]                  -- Alphabetic character
$alphanum   = [$alpha$digit]            -- Alphanumeric character

tokens :-

$digit+                                 { IntTok .read }
($digit* "." $digit+)                   { RealTok .read }
"E"                                     { const ETok }
"PI"                                    { const PiTok }
"PHI"                                   { const PhiTok }
"AVOGADRO"                              { const AvogadroTok }
"+="                                    { const PlusEqTok }
"-="                                    { const MinusEqTok }
"*="                                    { const MultEqTok }
"/="                                    { const DivEqTok }
"%="                                    { const ModEqTok }
"--"                                    { const DecrementTok }
"+"                                     { const PlusTok }
"-"                                     { const MinusTok }
"*"                                     { const MultTok }
"/"                                     { const DivTok }
"("                                     { const LParenTok }
")"                                     { const RParenTok }
"!"                                     { const NotTok }
"**"                                    { const ExpTok }
"%"                                     { const ModTok }
"=="                                    { const EqTok }
">="                                    { const GeqTok }
"<="                                    { const LeqTok }
">"                                     { const GTTok }
"<"                                     { const LTTok }
"!="                                    { const NeqTok }
"Math.sqrt"                             { const MathSqrtTok }
"true"                                  { const TrueTok }
"false"                                 { const FalseTok }
"null"                                  { const NullTok }
"if"                                    { const IfTok }
"else"                                  { const ElseTok }
"let"                                   { const LetTok }
"const"                                 { const ConstTok }
"for"                                   { const ForTok }
"="                                     { const AssignTok }
"&&"                                    { const AndTok }
"||"                                    { const OrTok }
";"                                     { const SemicolonTok}
"fn"                                     { const FnTok}
"function"                              { const FunctionTok}
"=>"                                    { const ArrowTok}
"toString"                              { const ToStringTok}
"typeof"                                { const TypeOfTok}
"print"                                 { const PrintTok}
"int"                                   { const IntTypeTok}
"bool"                                  { const BoolTypeTok}
"string"                                { const StringTypeTok}
"in"                                    { const InTok}
"len"                                   { const LenTok}
","                                     { const CommaTok}
"return"                                { const ReturnTok}
"&"                                     { const RefTok }
"while"                                 { const WhileTok }
"match"                                 { const MatchTok }
"|"                                     { const PipeTok }
"class"                                 { const ClassTok }
"new"                                   { const NewTok }
"this"                                  { const ThisTok }
"."                                     { const DotTok }
"{"                                     { const LBraceTok }
"}"                                     { const RBraceTok }
"["                                     { const LBracketTok }
"]"                                     { const RBracketTok }
"?"                                     { const QuestionTok }
":"                                     { const ColonTok }
"_"                                     { const WildcardTok }
"++"                                    { const IncrementTok }
"err"                                   { const ErrTok }
"isErr"                                 { const IsErrTok }

[A-Z][a-zA-Z0-9]*                       { CnameTok }
[a-z][a-zA-Z0-9_]*                      { VarTok }

\"[^\"]*\"                              { StringTok }

$white+                                 ;
"//" [^\n]*                             ;

{
data Token = PlusTok | MinusTok | MultTok | DivTok | ExpTok | ModTok | MathSqrtTok
              | PlusEqTok | MinusEqTok | MultEqTok | DivEqTok | ModEqTok | DecrementTok
              | AndTok | OrTok | NotTok
              | TrueTok | FalseTok | NullTok
              | SemicolonTok
              | VarTok String
              | IntTok Int | RealTok Double
              | StringTok String
              | LetTok | ConstTok | ForTok | AssignTok
              | ETok | PiTok | PhiTok | AvogadroTok
              | IfTok | ElseTok
              | LParenTok | RParenTok | LBracketTok | RBracketTok
              | EqTok | GeqTok | LTTok | LeqTok | GTTok | NeqTok
              | FnTok | FunctionTok | ArrowTok | ToStringTok | TypeOfTok
              | PrintTok | IntTypeTok | BoolTypeTok | StringTypeTok | InTok | LenTok
              | CommaTok | ReturnTok
              | RefTok | WhileTok | MatchTok | PipeTok
              | NewTok | DotTok | LBraceTok | RBraceTok | CnameTok String
              | ClassTok | ThisTok
              | QuestionTok | ColonTok | WildcardTok | IncrementTok
              | ErrTok | IsErrTok
              deriving (Show, Eq, Ord)

scanTokens :: String -> Maybe [Token]
scanTokens str = go ('\n',[],str)
  where go inp@(_,_bs,str) =
          case alexScan inp 0 of
                AlexEOF -> Just []
                AlexError err -> traceShow err $ Nothing
                AlexSkip  inp' len     -> go inp'
                AlexToken inp' len act -> fmap ((act (take len str)):) (go inp')
}
