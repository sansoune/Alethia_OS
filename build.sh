#!/bin/bash

cargo build --package kernel --target x86_64-unknown-none
cargo build --package bootloader --target x86_64-unknown-uefi
make run
make clean