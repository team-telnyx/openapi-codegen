# Generates code from a JSON OpenAPI spec
.PHONY: from-json
from-json:
	@cargo run -- json < openapi.json > openapi.py

# Generates code from a YAML OpenAPI spec
.PHONY: from-yaml
from-yaml:
	@cargo run -- yaml < openapi.yaml > openapi.py

# Updates Python dependencies (for generated code)
.PHONY: update
update: requirements.in
	@pip-compile \
		--output-file=requirements.txt \
		--generate-hashes \
		--quiet \
		--upgrade \
		$^

# Generate a lockfile for Python dependencies (for generated code)
.PHONY: lock
lock: requirements.in
	@pip-compile \
		--output-file=requirements.txt \
		--generate-hashes \
		--quiet \
		$^

# Install Python dependencies (for generated code)
.PHONY: deps
deps: requirements.txt
	@pip \
		--disable-pip-version-check \
		install \
			--no-deps \
			--require-hashes \
			--requirement $^

target/python/doc/openapi.html: openapi.py
	@pdoc3 \
		--force \
		--html \
		--output-dir target/python/doc \
		$^

# Generate documentation from generated Python code
.PHONY: doc
doc: target/python/doc/openapi.html

# Open the generated python documenation in a browser
.PHONY: doc-open
doc-open: target/python/doc/openapi.html
	@xdg-open $^

# Clean up build artifacts
.PHONY: clean
clean:
	@cargo clean

# Download the Pet Store OpenAPI spec for testing
.PHONY: petstore
petstore:
	curl -O "https://petstore3.swagger.io/api/v3/openapi.json"
