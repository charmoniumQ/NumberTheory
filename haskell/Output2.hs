import System.Environment
import Data.List
import K_ary

main :: IO()
main = do
    args <- getArgs
    let
        maxK = (read $ args !! 0) :: Int
        maxN = (read $ args !! 1) :: Int
        header = intercalate "," ["k", "mu(p^0)", "mu(p^1)", "..."]
        body = [intercalate "," $ map (\i -> show $ k_mobius i k) [0..maxN]
               | k <- [0..maxK]]
        text = header ++ "\n" ++ intercalate "\n" body
    putStrLn text
