main :: IO ()
main = do
  contents <- readFile "input.txt"
  let result = process contents
  putStrLn result

process :: String -> String
process input = show (countIncs measurements)
  where
    measurements = tripleSums $ map read $ lines input

tripleSums :: (Num a) => [a] -> [a]
tripleSums xs = map (\(a, b, c) -> a + b + c) (zip3 xs (drop 1 xs) (drop 2 xs))

countIncs :: (Num a, Ord a) => [a] -> Int
countIncs xs = length $ filter (> 0) deltas
  where
    deltas = map (\(a, b) -> b - a) pairs
    pairs = zip xs (drop 1 xs)