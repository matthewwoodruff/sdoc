#! /usr/bin/env bash 
set -e

declare target=$TARGET
default_target=$(head -n 1 <<<$(rustup show) | cut -d ' ' -f 3)

echo "Default target is: $default_target"

if [[ -z "$target" ]];
then
  target=$default_target
elif [[ "$target" != "$default_target" ]];
then
  rustup target add $target
fi

echo Building for $target

declare path=target/$target/release

cargo clean 
cargo test --all --target $target
cargo build --release --target $target
./test/run.sh $path/sdoc

mkdir -p dist
declare build=${TRAVIS_TAG:-SNAPSHOT}
tar czf dist/sdoc-$build-$target.tar.gz -C $path sdoc

exit 0
