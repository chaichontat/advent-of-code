module Day01 where

import qualified Data.IntSet                   as IS
import           Data.List                      ( sort
                                                , tails
                                                , transpose
                                                )
import           Data.Tuple                     ( swap )
import           Lib                            ( fst3
                                                , lst3
                                                , snd3
                                                , windows
                                                )

parseInt :: String -> Int
parseInt x | head x == '+' = (read :: String -> Int) $ drop 1 x
           | otherwise     = (read :: String -> Int) x

day01a :: [String] -> Maybe Int
day01a s = Just $ sum $ map parseInt s

cumsum :: [Int] -> (Int, [Int])
cumsum xs = (last summed, init summed) where summed = scanl (+) 0 xs

divmodidx :: Integral a => a -> [a] -> [(a, a, a)]
divmodidx sum' xs =
  [ (md, qt, i) | (i, (qt, md)) <- zip [0 ..] $ map (`divMod` sum') xs ]

filterMap :: Integral a => [(a, a, a)] -> [(a, a, a)]
filterMap xs =
  [ (snd3 y - snd3 prev, lst3 prev, lst3 y)
  | [prev, y] <- windows 2 xs
  , fst3 prev == fst3 y
  ]

day01b_ :: [Int] -> Int
day01b_ xs = cs !! idx
 where
  idx       = lst3 . minimum $ (filterMap . sort . divmodidx sum) cs
  (sum, cs) = cumsum xs

day01b :: [String] -> Maybe Int
day01b xs = Just $ day01b_ $ map parseInt xs
