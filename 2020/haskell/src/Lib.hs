module Lib
  ( someFunc
  ) where

f :: [String] -> [Integer]
f = map read

someFunc :: IO ()
someFunc = do
  x <- readFile "../data/day01.txt"
  let s = f . lines $ x
  print s
