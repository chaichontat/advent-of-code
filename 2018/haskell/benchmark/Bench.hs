import           Criterion.Main                 ( bench
                                                , bgroup
                                                , defaultMain
                                                , nf
                                                , nfIO
                                                , whnf
                                                )
import           Day01                          ( day01a
                                                , day01b
                                                )

runBench :: String -> ([String] -> Maybe Int) -> IO ()
runBench name f = do
  x <- readFile $ "../data/" ++ name ++ ".txt"
  let _ = f $ lines x
  return ()

main :: IO ()
main = do
  defaultMain
    [ bgroup
        "adv"
        [ bench "1a" $ nfIO $ runBench "day01" day01a
        , bench "1b" $ nfIO $ runBench "day01" day01b
        ]
    ]
