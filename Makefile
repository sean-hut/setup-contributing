# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

#######
# rules
#######

all: rust-all git-diff-check

rust-all: format clippy build test

.PHONY: format
format:
> cargo fmt -- --check --files-with-diff

.PHONY: clippy
clippy:
> cargo clippy -- --deny clippy::all

.PHONY: build
build:
> cargo build

.PHONY: test
test:
> cargo test

.PHONY: git-diff-check
git-diff-check:
> git diff --check

.PHONY: clean
clean:
> cargo clean
