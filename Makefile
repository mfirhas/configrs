install:
	@echo "install dependencies..."
	@cargo install grcov

cover:
	@echo "generate code coverage"
	@CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='z-%p-%m.profraw' cargo test -- --test-threads=1
	@grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./lcov.info
