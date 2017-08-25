module Util (sorted_isect) where

sorted_isect :: [Int] -> [Int] -> [Int]
sorted_isect [] _ = []
sorted_isect _ [] = []
sorted_isect (x:xs) (y:ys) =
  if x == y
  then [x] ++ (sorted_isect xs ys) -- take off both heads
  else
    if x > y
    then sorted_isect (x:xs) (ys) -- takes off ys head
    else sorted_isect (xs) (y:ys) -- takes off xs head

