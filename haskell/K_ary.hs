module K_ary (k_divides, k_coprime, k_gcd, k_divisors, k_characteristic, infinitary_divisors, k_mobius) where
import Util
import Data.Function.Memoize

-- k-ary.rkt
k_divides :: Int -> Int -> Int -> Bool
k_divides d n 0 = d <= n
k_divides d n k = d <= n && k_coprime d (n - d) (k - 1)

k_coprime :: Int -> Int -> Int -> Bool
k_coprime a b k = 0 == k_gcd a b k

k_gcd :: Int -> Int -> Int -> Int
k_gcd a b k = last $ sorted_isect (k_divisors a k) (k_divisors b k)

k_divisors :: Int -> Int -> [Int]
k_divisors = memoize2 k_divisors_
k_divisors_ :: Int -> Int -> [Int]
k_divisors_ n k = [d | d <- [0..n], k_divides d n k]

infinitary_divides :: Int -> Int -> Bool
infinitary_divides b 0 = b == 0
infinitary_divides a b = k_divides a b (b - 1)

infinitary_divisors :: Int -> [Int]
infinitary_divisors n = [d | d <- [0..n], infinitary_divides d n]

-- characteristic.rkt
first_non_div :: Int -> Int -> Int
first_non_div a b = head [k | k <- [0,2..(1+b)], not $ k_divides a b k]

first_div :: Int -> Int -> Int
first_div a b = head [k | k <- [1,3..(1+b)], k_divides a b k]

k_characteristic :: Int -> Int -> Int
k_characteristic a b =
    if infinitary_divides a b
    then first_div a b
    else first_non_div a b

-- Mobius stuff

k_mobius :: Int -> Int -> Int
k_mobius 0 k = 1
k_mobius n k = 0 - (sum [k_mobius i k | i <- [0..n-1], k_divides i n k])
