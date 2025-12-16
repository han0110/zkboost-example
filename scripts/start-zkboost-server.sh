#!/bin/bash

RUST_LOG=info \
  ERE_IMAGE_REGISTRY=ghcr.io/eth-act/ere \
  zkboost-server --config ./config.toml --port 3001
