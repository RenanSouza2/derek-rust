run r: main
	./main

build b: main

main: src/main.rs
	rustc $^

.PHONY: clean c
clean c:
	cargo clean