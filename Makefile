install:
	@echo "install dependencies..."
	@cargo install grcov

test:
	@echo "running test"
	@CARGO_INCREMENTAL=0 \
	 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
	 RUSTDOCFLAGS="-Cpanic=abort" \
	 cargo +nightly build
	@CARGO_INCREMENTAL=0 \
	 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
	 RUSTDOCFLAGS="-Cpanic=abort" \
	 cargo +nightly test -- --test-threads=1

cover:
	@echo "generate code coverage"
	@grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/lcov.info
