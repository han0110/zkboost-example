#!/bin/bash

docker run \
    -v ./fibonacci:/fibonacci \
    -e RUST_LOG=info \
    ghcr.io/eth-act/ere/ere-compiler-sp1:0.0.15-d15d36a \
    --compiler-kind \
    rust-customized \
    --guest-path \
    /fibonacci \
    --output-path \
    /fibonacci/fibonacci-sp1
