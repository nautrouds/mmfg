#!/bin/bash
set -e

echo "--- 1. Building Binaries ---"
(
    cd rust
    cargo build --release
)
mkdir -p bin

(
    go build -o bin/cross_lang_integration_test ./tests/integration/main.go
)

echo "--- 2. Running Integration Test Orchestrator ---"
./bin/cross_lang_integration_test

echo "--- Tests finished ---"

