module Main where

import Data.Char (ord)
import Data.List.Split
import System.IO
  ( IOMode (ReadMode),
    hClose,
    hGetContents,
    openFile,
  )

solve :: String -> Int
solve input = fold construct . splitOn "," input

construct :: [String] -> [(String, Int, Int)]
construct str:strs = do
  prev = find  

handleFile :: String -> (String -> Int) -> IO ()
handleFile fileName f = do
  handle <- openFile fileName ReadMode
  contents <- hGetContents handle
  print $ f contents
  hClose handle