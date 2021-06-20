module Day01 where

import           Control.Monad (guard)
import qualified Data.IntSet   as IS
import           Data.Maybe    (listToMaybe)
import           Test.Hspec

findPair :: Int -> IS.IntSet -> Maybe Int
findPair goal xs =
  listToMaybe $ do
    x <- IS.toList xs
    let y = goal - x
    guard (y `IS.member` xs)
    pure (x * y)

test :: IO ()
test = do
  x <- readFile "../data/day01.txt"
  let s = map (read :: String -> Int) . lines $ x
  print $ findPair 2020 $ IS.fromList s
