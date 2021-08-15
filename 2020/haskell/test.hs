module Test where

import           Data.Monoid

sayHello :: String -> IO ()
sayHello x = putStrLn ("Hello, " ++ x ++ "!")

printInc :: (Show a, Num a) => a -> IO ()
printInc n = print plusTwo
  where
    plusTwo = n + 2

y = [1, 2, 3]

isPalindrome x = reverse x == x

myAbs x =
  if x > 0
    then x
    else -x

data Purim
  = Pluem
  | Pl

instance Eq Purim where
  (==) Pluem Pluem = True
  (==) Pl Pl       = True
  (==) _ _         = False

data EitherOr a b
  = Hello a
  | Goodbye b

instance (Eq a, Eq b) => Eq (EitherOr a b) where
  (==) (Hello a) (Hello a')     = a == a'
  (==) (Goodbye b) (Goodbye b') = b == b'
  (==) _ _                      = False

lms = [Just "Ave", Nothing, Just "woohoo"]

replaceWithP = const 'p'

replaceWithP lms
