module Math where

import Data.Int

--      hey
-- desc
-- a
--d
add :: Int64 -> Int64 -> Int64
-- a
add a b = a + b

-- Fonction 2
-- a
multiply :: Int64
  -> Int64
  -> Int64
multiply a b = a * b

factorial :: Int64 -> Int64
factorial 0 = 1
factorial n = n * factorial (n - 1)