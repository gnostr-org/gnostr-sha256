CFLAGS = -O3 -Wall -Wextra -Wpedantic -std=c99

gnostr-sha256-test: gnostr-sha256-test.o gnostr-sha256.o

gnostr-sha256-test.o: gnostr-sha256.h gnostr-sha256-test.c

gostr-sha256.o: gnostr-sha256.h gnostr-sha256.c

.PHONY: all
all: gnostr-sha256-test
	./gnostr-sha256-test

.PHONY: clean
clean:
	rm -f gnostr-sha256-test *.o
