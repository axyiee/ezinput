#!/bin/bash

cd "${0%/*}"

cargo run --features "bevy/dynamic bevy/default" --example singleplayer