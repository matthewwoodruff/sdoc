#! /bin/bash

echo Cargo publish 

cargo login "$CRATES_API_TOKEN"
cargo publish

exit 0
