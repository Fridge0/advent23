module Main where

import Data.Char (ord)
import Data.List.Split
import System.IO

hash :: String -> Int
hash str = hashMid $ reverse str

hashMid :: String -> Int
hashMid [] = 0
hashMid str = (prevValue + fromASCII (head str)) * 17 `mod` 256
  where
    prevValue = hashMid (drop 1 str)

fromASCII :: Char -> Int
fromASCII = ord

solve :: String -> Int
solve str = sum $ map hash $ splitOn "," str

main :: IO ()
main = do
  print $ solve "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
  handleFile "src/day15.txt" solve

handleFile :: String -> (String -> Int) -> IO ()
handleFile fileName f = do
  handle <- openFile fileName ReadMode
  contents <- hGetContents handle
  print $ f contents
  hClose handle