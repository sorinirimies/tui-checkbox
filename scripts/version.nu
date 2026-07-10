#!/usr/bin/env nu
# Prints the current crate version from Cargo.toml.
# Usage: nu scripts/version.nu

open Cargo.toml | get package.version | print
