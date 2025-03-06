import Data.List

main = do
        contents <- readFile "input.txt"
        let [nums1, nums2] = transpose $ map mapInt $ map words $ lines contents
        print $ sum $ map (similarity nums2) nums1

mapInt :: [String] -> [Int]
mapInt vals = map (read::String->Int) vals

similarity ::  [Int] -> Int -> Int
similarity haystack needle = needle * length (filter (==needle) haystack)
