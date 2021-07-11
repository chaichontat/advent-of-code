import           Day01      (day01a)
import           Test.HUnit (assertEqual)

runtest :: String -> ([Int] -> Maybe Int) -> Int -> IO ()
runtest name proposed ans = do
  x <- readFile $ "../data/" ++ name ++ ".txt"
  let s = map (read :: String -> Int) . lines $ x
  assertEqual name (proposed s) (Just ans)
  putStrLn $ "Test " ++ name ++ " passed."

main :: IO ()
main = do
  runtest "day01" day01a 605364
  runtest "day01" day01a 605364
