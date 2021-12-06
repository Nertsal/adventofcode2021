main :: IO ()
main = do
  contents <- readFile "input.txt"
  let result = process contents
  putStrLn result

process :: String -> String
process input = show (countIncs measurements)
  where
    measurements = map read $ lines input

countIncs :: (Num a, Ord a) => [a] -> Int
countIncs xs = length $ filter (> 0) deltas
  where
    deltas = map (\(a, b) -> b - a) pairs
    pairs = zip xs (drop 1 xs)