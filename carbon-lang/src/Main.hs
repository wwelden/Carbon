module Main where
import Lexer
import Parser
import Evaluator
import Evaluator (Value(..))
import System.IO
import Control.Monad
import Control.Exception (catch, IOException)

main :: IO ()
main = do
    putStrLn "Carbon Programming Language Interpreter"
    putStrLn "Type 'quit' to exit"
    repl (Context [] [] [])

repl :: Context -> IO ()
repl ctx = do
    putStr "> "
    hFlush stdout
    result <- catch (Just <$> getLine) (\(_ :: IOException) -> return Nothing)
    case result of
        Nothing -> putStrLn "\nGoodbye!"
        Just input ->
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
                                    case val of
                                        PrintVal output -> do
                                            putStrLn output
                                            putStrLn "null"
                                        _ -> putStrLn $ showVal val
                                    repl newCtx
