#!/usr/bin/env bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <gif-file>"
    exit 1
fi

GIF_FILE=$1

if [ ! -f "$GIF_FILE" ]; then
    echo "Error: File '$GIF_FILE' not found!"
    exit 1
fi

cargo build --release
samply record ./target/release/jif-render "$GIF_FILE"