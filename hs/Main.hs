{-# LANGUAGE ViewPatterns #-}

module Main where

import Control.Monad (replicateM_)
import System.Environment (getArgs)

explosive :: (Applicative m, Foldable m) => m () -> m ()
explosive xs = replicateM_ (length xs) xs

main :: IO ()
main = do
  [read -> count] <- getArgs
  print . explosive $ replicate count ()
