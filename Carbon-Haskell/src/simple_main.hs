module Main where
import Lexer
import Parser
import Evaluator
import Evaluator (Value(..))
import System.IO

main :: IO ()
main = do
    putStrLn "Carbon Programming Language Interpreter"
    putStrLn "Type 'quit' to exit"
    putStr "> "
    hFlush stdout
    input <- getLine
    case scanTokens input of
        Nothing -> putStrLn "Lexer error"
        Just tokens ->
            case parse tokens of
                Nothing -> putStrLn "Parser error"
                Just stmt ->
                    case evalStmt (Context [] [] []) stmt of
                        Nothing -> putStrLn "Runtime error"
                        Just (newCtx, val) -> do
                            case val of
                                PrintVal output -> do
                                    putStrLn output
                                    putStrLn "null"
                                _ -> putStrLn $ showVal val
    putStrLn "Goodbye!"
