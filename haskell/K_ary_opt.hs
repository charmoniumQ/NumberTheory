module K_ary (k_characteristic_triangle) where

import Data.Primitive.Types

index :: (Int, Int) -> Int
index (a, b) = a * (a + 1) + b

-- Get the nth row (starting from one)
k_characteristic_triangle :: Int -> [[Int]]
k_characteristic_triangle n =
  let t = replicate (index (n+1, 0)) -1
