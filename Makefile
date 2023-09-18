# Unfortunately, Cargo doesn't properly facilitate certain maintenance tasks this project is reliant on.

# Generate bindings
gen-bindings:
	rm -rf src/backend/c/bindings.rs
	bindgen
