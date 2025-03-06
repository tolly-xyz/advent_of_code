main = do
        contents <- readFile "input.txt"
        let reports = map mapInt $ map words $ lines contents
        print $ length $ filter (==True) $ map safe reports

safe :: [Int] -> Bool
safe nums = all (\(x, y) -> ((y - x > 0) && (y - x < 4))) ||
            all (\(x, y) -> ((x - y > 0) && (x - y < 4))) $
            pairs nums

pairs :: [Int] -> [(Int, Int)]
pairs nums = zip nums (tail nums)

mapInt :: [String] -> [Int]
mapInt vals = map (read::String->Int) vals

