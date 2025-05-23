module Main where
import Lexer
import Parser
import Evaluator
import System.IO
import Control.Monad

main :: IO ()
main = do
    putStrLn "Carbon Programming Language Interpreter"
    putStrLn "Type 'quit' to exit"
    repl (Context [] [] [])

repl :: Context -> IO ()
repl ctx = do
    putStr "> "
    hFlush stdout
    input <- getLine
    if input == "quit"
        then putStrLn "Goodbye!"
        else do
            case scanTokens input of
                Nothing -> do
                    putStrLn "Lexer error"
                    repl ctx
                Just tokens ->
                    case parse tokens of
                        Nothing -> do
                            putStrLn "Parser error"
                            repl ctx
                        Just stmt ->
                            case evalStmt ctx stmt of
                                Nothing -> do
                                    putStrLn "Runtime error"
                                    repl ctx
                                Just (newCtx, val) -> do
                                    putStrLn $ showVal val
                                    repl newCtx
