import Criterion.Main
import Data.Function.Memoize

-- Let the runtime of this funciton be f(d, n, k)
k_divides :: Int -> Int -> Int -> Bool

-- f(d, n, 0) = 1
k_divides d n 0 = d <= n

-- f(d, n, k) = g(d, n - d, k - 1)
k_divides d n k = d <= n && k_coprime d (n - d) (k - 1)

-- Runs in g(a, b, k) time
k_coprime :: Int -> Int -> Int -> Bool

-- g(a, b, k) = g(a, b, k)
k_coprime a b k = 0 == k_gcd a b k

-- Let the runtime of this function be g(a, b, k)
k_gcd :: Int -> Int -> Int -> Int

-- g(a, b, k) = h(a, k) + h(b, k)
-- Computing the intersection is linear with both lists, probably much less than h(a, k),
-- therefore, I have omitted the time it takes to compute the intersection, max(a, b).
k_gcd a b k = last $ isect (k_divisors a k) (k_divisors b k)

-- Let the runtime of this function be h(n, k)
k_divisors :: Int -> Int -> [Int]
k_divisors = memoize2 k_divisors_
k_divisors_ :: Int -> Int -> [Int]

-- h(n, k) = sum [f(d, n, k) | d <- [0..n]]
k_divisors_ n k = [d | d <- [0..n], k_divides d n k]

-- f(d, n, k) = g(d, n-d, k-1) = g(d, n-d, k-1) = h(d, k-1) + h(n-d, k-1)
-- = sum [f(da, d, k-1) | da <- [0..d]] + sum [f(db, n-d, k-1) | db <- [0..n-d]]
-- < sum [f(da, b, k-1) | da <- [0..b]] + sum [f(db, b, k-1) | db <- [0..b]] where b = max(d, n-d) (note, this is an upper-bound (I think it is relatively tight))
-- = 2 * sum [f(da, b, k-1) | da <- [0..b]] where b = max(d, n-d)
-- = 2 * 2 * sum [f(da, b, k-1) | da <- [0..b/2]] where b = max(d, n-d) by a symmetry argument
-- (Since f(a, b, k) = f(b, a, k), so each term except perhaps the middle one is double-counted)

-- f(d, n, k) ~ j(max(d, n-d), k), to reduce the number of parameters
-- j(b, k) = 4 * sum [j(b - da, k-1) | da <- [0..b/2]]
-- 2*(b/2 * min [j(b - da, k-1) | da <- [0..b/2]]) <= j(b, k) <= 2*(b/2 * max [j(b - da, k-1) | da <- [0..b/2]])
--    b  *  j(b/2, k-1)                            <= j(b, k) <=    b  *  j(b, k-1)
-- and j(b, 0) = 1
-- b^(log_2 k)<= j(b, k) <= b^k

-- computes the intersection of two sorted lists
isect :: [Int] -> [Int] -> [Int]
isect [] _ = []
isect _ [] = []
isect (x:xs) (y:ys) =
  if x == y
  then (isect xs ys) ++ [x]
  else
    if x > y
    then isect (x:xs) (ys)
    else isect (xs) (y:ys)

-- tests if two sorted lists are disjoint
-- disjoint a b = null $ isect a b
-- except it doesn't have to compute the WHOLE intersection
disjoint :: [Int] -> [Int] -> Bool
disjoint [] _ = False
disjoint _ [] = False
disjoint (x:xs) (y:ys) =
  if x == y
  then True
  else
    if x > y
    then disjoint (x:xs) (ys)
    else disjoint (xs) (y:ys)

test :: (Int, Int) -> [[Int]]
test (maxk, maxn) = [k_divisors n k | k <- [0..maxk], n <- [0..maxn]]


main = defaultMain [bench "thing 1" $ nf test (100, 300)]
-- ghc -O3 --make k-ary && ./k-ary
-- min     avg     max   (4 samples)
-- 843 ms  843 ms  843 ms
