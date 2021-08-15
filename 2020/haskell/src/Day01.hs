module Day01 where

import           Control.Monad (guard)
import qualified Data.IntSet   as IS
import           Data.Maybe    (listToMaybe)

findPair :: Int -> IS.IntSet -> Maybe Int
findPair goal xs =
  listToMaybe $ do
    x <- IS.toList xs
    let y = goal - x
    guard (y `IS.member` xs)
    pure (x * y)

day01a :: [Int] -> Maybe Int
day01a s = findPair 2020 $ IS.fromList s
