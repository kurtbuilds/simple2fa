set dotenv-load := true

help:
    @just --list --unsorted

build:
    cargo build --lib
alias b := build

release:
    cargo build --lib --release

bootstrap:
    cargo install cargo-bump

test *args:
    cargo test {{args}}
alias t := test

check:
    cargo check

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo $(dye -r ERROR) You have untracked changes. Commit your changes before bumping the version.
    cargo bump {{level}}
    cargo update # This bumps Cargo.lock
    git commit -am "Bump {{level}} version"
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch: test
    just version patch
    just publish

flamegraph:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -b flame --root
