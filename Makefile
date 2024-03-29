install:
	@echo "install dependencies..."
	@cargo install grcov

test:
	@echo "running test"
	@CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='z-%p-%m.profraw' cargo test -- --test-threads=1

cover:
	@echo "generate code coverage"
	# @grcov . -s . --binary-path ./target/debug/ -t coveralls --branch --ignore-not-existing -o ./lcov.info
	@grcov . --binary-path ./target/debug/ -t coveralls -s . --token ${{ secrets.COVERALLS_TOKEN }} > coveralls.json
