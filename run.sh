#!/bin/bash
cargo build
DISPLAY=:1 ./target/debug/sapphirewm &

Xephyr -br -ac -noreset -screen 1024x768 :1 &