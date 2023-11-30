.PHONY: run test

.SILENT:

run:
	cargo run --bin $(filter-out $@,$(MAKECMDGOALS))

test:
	cargo test --bin $(filter-out $@,$(MAKECMDGOALS))

%:
	@:
