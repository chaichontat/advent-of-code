import           Day01      (day01a, day01b)
import           Test.HUnit (assertEqual)

runtest :: String -> ([String] -> Maybe Int) -> Int -> IO ()
runtest name f ans = do
  x <- readFile $ "../data/" ++ name ++ ".txt"
  let s = lines x
  assertEqual name (f s) (Just ans)
  putStrLn $ "Test " ++ name ++ " passed."

main :: IO ()
main = do
  runtest "day01" day01a 454
  runtest "day01" day01b 566
