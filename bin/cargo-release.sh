#! /bin/bash

set -euv

echo Cargo publish 

cargo login "$CRATES_API_TOKEN"
cargo publish

exit 0
