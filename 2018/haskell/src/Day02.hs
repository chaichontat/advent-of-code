module Day02 where

import           Control.Monad
import           Control.Monad.ST
import           Data.Array.Base
import           Data.Array.ST
import           Data.Char
import           Data.List                      ( sort
                                                , tails
                                                , transpose
                                                )
import           Data.Maybe
import           Data.Tuple                     ( swap )
import           Lib                            ( fst3
                                                , lst3
                                                , snd3
                                                , windows
                                                )


day02a = (map . map) (subtract 97 . ord) ["abc"]

listtoUArray :: [Int] -> UArray Int Int
listtoUArray cs = runSTUArray $ do
    arr <- newArray (97, 97 + 26) 0
    forM_ [0 .. length cs - 1] $ \i -> do
        curr <- readArray arr i
        writeArray arr (cs !! i) (curr + 1)
    return arr



