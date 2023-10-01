ALL := rs/main hs/Main

.PHONY: all
all: $(ALL)

rs/main: rs/main.rs
	rustc -O $^ -o $@

hs/Main: hs/Main.hs
	ghc -O2 $^ -o $@

.PHONY: clean
clean:
	rm -f $(ALL) hs/Main.o hs/Main.hi
