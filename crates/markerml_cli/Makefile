
build:
	cargo build

test:
	cargo test

convert:
	cargo run -- convert --input $(input) --output $(output)

watch:
    cargo run -- watch --input $(input)

format:
	cargo fmt

lint:
	cargo clippy

clean:
	cargo clean

.PHONY: format, lint, build, clean, convert, watch
