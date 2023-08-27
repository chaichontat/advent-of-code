module Main where

import Data.Functor ((<&>))
import Data.List (sort)
import IO (readAdvent)
import Safe (maximumMay)
import Text.Read (readMaybe)

innerparser :: String -> Maybe [Int]
innerparser elfStr = mapM readMaybe $ lines elfStr

takelast :: Int -> [a] -> Maybe [a]
takelast n xs
  | n < 0 || n > length xs = Nothing
  | otherwise = Just $ drop (length xs - n) xs

part1 :: [[Int]] -> Maybe Int
part1 elves = maximumMay $ map sum elves

part2 :: [[Int]] -> Maybe Int
part2 = (fmap sum) . takelast 3 . sort . (map sum)

main :: IO ()
main = do
  parsed <- readAdvent "\n\n" "data/2022/day01.txt"
  let elves = mapM innerparser parsed
  let result = traverse (elves >>=) [part1, part2]
  maybe (print "Invalid input") print result