HAPPY = ~/.cabal/bin/happy
ALEX = ~/.cabal/bin/alex
GHC = ghc

all: carbon

carbon: Main.hs Parser.hs Lexer.hs Evaluator.hs
	$(GHC) -o carbon Main.hs

Parser.hs: Parser.y
	$(HAPPY) Parser.y

Lexer.hs: Lexer.x
	$(ALEX) Lexer.x

clean:
	rm -f *.hi *.o Parser.hs Lexer.hs carbon

.PHONY: all clean
