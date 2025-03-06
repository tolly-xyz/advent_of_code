import Data.List

main = do
        contents <- readFile "input.txt"
        print $ sum $ map absDiff $ (uncurry zip) $ toTuple $ map sort $ transpose $ map mapInt $ map words $ lines contents

mapInt :: [String] -> [Int]
mapInt vals = map (read::String->Int) vals

toTuple :: [[Int]] -> ([Int], [Int])
toTuple [nums1, nums2] = (nums1, nums2)

absDiff :: (Int, Int) -> Int
absDiff (num1, num2) = abs(num1 - num2)
