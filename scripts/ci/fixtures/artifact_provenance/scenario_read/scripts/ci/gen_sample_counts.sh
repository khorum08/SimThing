#!/usr/bin/env bash
OUT_DEFAULT="scripts/ci/sample_counts.tsv"
cargo test -p fixture-crate --test sample_generator generator_cli
