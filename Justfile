set dotenv-load := true

bin_name := `rg "name = \"(.*)\"" -or '$1' Cargo.toml | head -n1`

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    @checkexec target/debug/{{bin_name}} $(fd -e rs) -- cargo build
    @target/debug/{{bin_name}} {{args}}

alias r := run

release:
    cargo build --release

install:
    cargo install --path .

bootstrap:
    cargo install cargo-bump

test:
    cargo test
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
