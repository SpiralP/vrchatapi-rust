#!/bin/bash
set -e

# Generate Client
rm -rf src/apis src/models docs

npx openapi-generator-cli generate \
    -g rust \
    --additional-properties=packageName=vrchatapi \
    --git-user-id=vrchatapi \
    --git-repo-id=vrchatapi-rust \
    -o . \
    -i https://raw.githubusercontent.com/vrchatapi/specification/gh-pages/openapi.yaml \
    --http-user-agent="vrchatapi-rust"
    #--global-property debugOperations=true

# Use 2021 edition rust
sed -i 's/^edition = "2018"/edition = "2021"/' Cargo.toml

# Add description and license to Cargo.toml
sed -i '/^name = "vrchatapi"/a description = "VRChat API Library for Rust"' Cargo.toml
sed -i '/^edition = "2021"/i license = "MIT"' Cargo.toml

# add "cookies" feature to reqwest
sed -i 's/^features = \[/features = \["cookies", /' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;
# Remove openapi version in every file
find src -type f -exec sed -i '/The version of the OpenAPI document/d' {} \;

# # remove Default derive macro from all types
find src -type f -exec sed -i 's/Default, //g' {} \;

cargo fmt

git apply configuration.patch
git apply two_factor.patch

cargo build
