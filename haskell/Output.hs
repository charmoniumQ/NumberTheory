import System.Environment
import Data.List
import K_ary

-- % ghc -O3 Output2.hs -o Output && echo "done" && ./Output 100

main :: IO()
main = do
    args <- getArgs
    let
        maxRow = (read $ head args) :: Int
        triangle = [[k_characteristic a b | a <- [0..b]] | b <- [0..maxRow]] :: [[Int]]
        text = intercalate "\n" $ map (\row -> intercalate "," $ map show row) triangle
    putStrLn text
