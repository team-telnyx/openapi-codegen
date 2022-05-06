# Shortcuts for Python commands

.PHONY: from-json
from-json:
	@cargo run -- json < openapi.json > openapi.py

.PHONY: from-yaml
from-yaml:
	@cargo run -- yaml < openapi.yaml > openapi.py

.PHONY: update
update: requirements.in
	@pip-compile \
		--output-file=requirements.txt \
		--generate-hashes \
		--quiet \
		--upgrade \
		$^

.PHONY: lock
lock: requirements.in
	@pip-compile \
		--output-file=requirements.txt \
		--generate-hashes \
		--quiet \
		$^

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

# Easier name for the above
.PHONY: doc
doc: target/python/doc/openapi.html

.PHONY: doc-open
doc-open: target/python/doc/openapi.html
	@xdg-open $^

.PHONY: clean
clean:
	@cargo clean
