#!/usr/bin/env bash

set -euo pipefail

RUST_BACKTRACE=1 cargo run < input.txt
