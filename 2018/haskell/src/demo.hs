import           Data.Maybe


prependCats :: [Char] -> [Char]
prependCats = (++) "cats"

-- What if we want to apply this to wrapped args?
-- Use functor, right?

prependCatsF = (++) <$> Just "cats"

-- Shit, we have a partial function wrapped in a context.
-- Cannot apply functor again
-- Need to แกะ function. Only then can we apply functor again.

prependCatsFUnwrapped = fmap fromMaybe (const "a") prependCatsF $ Just "dogs"

-- Now, we need to specify what happens if the first arg is Nothing.
-- Since the function must return [Char], we cannot put Nothing here.
-- Bad bad bad.


prependCatsFApp = ((++) <$> Just "cats") <*> Just "dogs"


