#! /usr/bin/env bash 
set +ex
target=${TARGET:?Target expected}
cargo clean 
cargo test --target $target --all 
cargo build --release --target $target
./test/run.sh target/$target/release/sdoc


exit 0
