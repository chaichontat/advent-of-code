module Main where

import Advent (AoC (AoCInput), AoCError, AoCOpts (_aCache, _aThrottle), dayInt, defaultAoCOpts, mkDay_, runAoC)
import Advent.Types (Day)
import Configuration.Dotenv (defaultConfig, loadFile)
import Data.Text (Text)
import System.Environment (getEnv)

newtype SessionKey = Key String

aocOpts :: SessionKey -> Advent.AoCOpts
aocOpts (Key key) = (Advent.defaultAoCOpts 2022 key) {Advent._aCache = Just ".", Advent._aThrottle = 1}

fetchInput :: Day -> SessionKey -> IO Text
fetchInput day key = do
  putStrLn $ "Fetching input for day " <> show (dayInt day) <> "...\n"
  response <- Advent.runAoC (aocOpts key) (Advent.AoCInput day)
  case response of
    Left err -> fail $ "Error response from API: " <> show err
    Right input -> pure input

getter :: Integer -> String -> IO Text
getter day key = fetchInput (mkDay_ day) $ Key key

main :: IO ()
main = do
  loadFile defaultConfig
  key <- getEnv "SESSION_KEY"
  mapM_ (`getter` key) $ enumFromTo 1 25
