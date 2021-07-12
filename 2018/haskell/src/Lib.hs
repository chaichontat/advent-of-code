module Lib
  ( fst3
  , snd3
  , lst3
  , windows
  ) where

import           Data.List                      ( tails )


fst3 :: (a, b, c) -> a
fst3 (x, _, _) = x

snd3 :: (a, b, c) -> b
snd3 (_, x, _) = x

lst3 :: (a, b, c) -> c
lst3 (_, _, x) = x

windows :: Int -> [a] -> [[a]]
windows n xs = map (take n) $ tails xs
