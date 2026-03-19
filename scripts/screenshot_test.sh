#!/usr/bin/env bash
set -euo pipefail

SCREENSHOT="/tmp/rusty_runner_screenshot.png"
GAME_PID=""

cleanup() {
    if [ -n "$GAME_PID" ] && kill -0 "$GAME_PID" 2>/dev/null; then
        kill "$GAME_PID" 2>/dev/null || true
        wait "$GAME_PID" 2>/dev/null || true
    fi
    rm -f "$SCREENSHOT"
}
trap cleanup EXIT

echo "Building game..."
cargo build --release --bin game

echo "Launching game..."
./target/release/game &
GAME_PID=$!

echo "Waiting for window to render..."
sleep 4

if ! kill -0 "$GAME_PID" 2>/dev/null; then
    echo "FAIL: Game process exited before screenshot"
    exit 1
fi

echo "Capturing screenshot..."
import -window root "$SCREENSHOT"

if [ ! -f "$SCREENSHOT" ]; then
    echo "FAIL: Screenshot file not created"
    exit 1
fi

echo "Analyzing screenshot..."
STDDEV=$(identify -verbose "$SCREENSHOT" | grep "standard deviation" | head -1 | awk '{print $3}')

if [ -z "$STDDEV" ]; then
    echo "FAIL: Could not read pixel standard deviation"
    exit 1
fi

THRESHOLD=5
STDDEV_INT=${STDDEV%%.*}

if [ "$STDDEV_INT" -lt "$THRESHOLD" ]; then
    echo "FAIL: Screenshot appears all-black (stddev=$STDDEV < $THRESHOLD)"
    exit 1
fi

echo "PASS: Screenshot has visual content (stddev=$STDDEV)"
