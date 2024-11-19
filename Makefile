runx:
	source .env && cargo run -p alloy-rs-practice --bin $(filter-out $@,$(MAKECMDGOALS))

# This prevents make from treating the argument as a target
%:
	@: