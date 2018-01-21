#! /usr/bin/env bash 
set -e

declare target=$TARGET
default_target=$(rustc +nightly -Z unstable-options --print target-spec-json | jq -r '."llvm-target"')

if [[ -z "$target" ]];
then
  target=$default_target
elif [[ "$target" != "$default_target" ]];
then
  rustup target add $target
fi

echo Building for $target

declare exe=target/$target/release/sdoc

cargo clean 
cargo test --all --target $target
cargo build --release --target $target
./test/run.sh $exe

mkdir -p dist
tar czf dist/sdoc-$build-$target.tar.gz $exe

exit 0
