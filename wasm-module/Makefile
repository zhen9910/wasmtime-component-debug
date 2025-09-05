CC=clang
CFLAGS=--target=wasm32 -O2 -nostdlib -Wl,--no-entry -Wl,--export=foo

all: foo.wasm

foo.wasm: foo.c
	$(CC) $(CFLAGS) foo.c -o foo.wasm

clean:
	rm -f foo.wasm
