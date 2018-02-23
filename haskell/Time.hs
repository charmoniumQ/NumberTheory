import Criterion.Main
import K_ary

test :: (Int, Int) -> [[Int]]
test (maxk, maxn) = [k_divisors n k | k <- [0..maxk], n <- [0..maxn]]

main = defaultMain [bench "thing 1" $ nf test (150, 400)]
-- % ghc -O3 --make Time.hs && ./Time
-- min     avg     max   (4 samples)
-- 843 ms  843 ms  843 ms
