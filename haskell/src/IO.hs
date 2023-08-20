module IO (readAdvent) where

import Control.Exception (IOException, catch)
import Data.List.Split (splitOn)
import System.IO (readFile)

readAdvent :: String -> FilePath -> IO [String]
readAdvent sep filePath = splitOn sep <$> catch (readFile filePath) handleIOException

handleIOException :: IOException -> IO String
handleIOException e = putStrLn ("Error reading file: " ++ show e) >> return ""