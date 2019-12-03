# Real targets
pkg/package.json:	Cargo.toml Cargo.lock src/*.rs
	nix-shell --run 'bash build.sh'
pkg/README.md:
	ln ../README.md pkg/
tests/node_modules:
	cd tests; npm i
docs/index.html:	pkg .jsdoc.json
	npx jsdoc --verbose -c .jsdoc.json --private --destination docs pkg/wasm_key_manager.js


.PHONY: pkg tests preview-package publish-package
# Alias
pkg:			pkg/package.json
docs:			docs/index.html

# Commands
tests:			pkg tests/node_modules
	cd tests; npm test

preview-package:	pkg pkg/README.md
	npm pack --dry-run ./pkg

publish-package:	pkg pkg/README.md
	npm publish --access public ./pkg

publish-docs:		pkg
	@echo "\nBuilding Key Manager docs"
	make docs
	ln -s docs v$$( cat ./pkg/package.json | jq -r .version )
	@echo "\nAdding Key Manager docs..."
	git add -f docs
	git add v$$( cat ./pkg/package.json | jq -r .version )
