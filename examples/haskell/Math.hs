module Math where

import Data.Int

-- Add function
add :: Int64 -> Int64 -> Int64
add a b = a + b

multiply :: Int64 -> Int64 -> Int64
multiply a b = a * b

factorial :: Int64 -> Int64
factorial 0 = 1
factorial n = n * factorial (n - 1)

testfn :: a -> a
testfn arg = arg