module Diamond (diamond) where

import Data.Char (chr, ord, toUpper, isAsciiUpper)
import Data.Maybe (fromMaybe)

diamond :: Char -> Maybe [String]
diamond x
      | isAsciiUpper x = Just (diamond' x 'A' [])
      | otherwise = Nothing

diamond' :: Char -> Char -> [String] -> [String]
diamond' max current xs
                   | dist > 0  = diamond' max next ys
                   | otherwise = ys ++ reverse (take (length ys - 1) ys)
                   where dist  = getCharDistance max current
                         next  = nextChar current
                         row   = [fromMaybe "" (intoDiamondRow max current)]
                         ys    = xs ++ row

intoDiamondRow :: Char -> Char -> Maybe String
intoDiamondRow max current = if diffToMax >= 0 && diffToCenter >= 0
                                then if diffToCenter > 0
                                    then Just (dotsToMax ++ [current] ++ dotsToCenter ++ ['.'] ++ dotsToCenter ++ [current] ++ dotsToMax)
                                    else Just (dotsToMax ++ [center] ++ dotsToMax)
                                else Nothing
                            where dotsToMax    = take diffToMax dots
                                  dotsToCenter = take (diffToCenter - 1) dots
                                  dots         = repeat '.'
                                  diffToMax    = getCharDistance max current
                                  diffToCenter = getCharDistance current center
                                  center       = 'A'

getCharDistance :: Char -> Char -> Int
getCharDistance x y = ord x - ord y

nextChar :: Char -> Char
nextChar 'Z' = 'Z'
nextChar c = chr (ord c + 1)

main :: IO ()
main = do putStrLn "Enter any char from a - z:"
          c <- getChar
          putStrLn ""
          putStrLn ""
          putStrLn "***diamond***"
          putStrLn $ unlines $ fromMaybe [] $ diamond $ toUpper c
          putStrLn "***diamond***"