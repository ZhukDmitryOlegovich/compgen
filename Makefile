all: bin/generator bin/calculator
bin/generator: $(wildcard generator/**/*.rs)
	mkdir -p bin
	cd generator && cargo build --release && cp target/release/generator ../bin
bin/calculator: $(wildcard calculator/**/*.rs)
	mkdir -p bin
	cd calculator && cargo build --release && cp target/release/calculator ../bin
clean:
	rm -rf ./bin
.PHONY: all clean