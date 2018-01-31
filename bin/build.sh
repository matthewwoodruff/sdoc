#! /usr/bin/env bash 
set -e

while getopts ":q" opt; do
  case "$opt" in
    q) quick_build='true';
  esac
done
shift $((OPTIND-1))

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

full_build_type=$(test -z "$quick_build" && echo 'release' || echo 'debug')

echo Building $full_build_type for $target

declare build_path=target/$target/$full_build_type
build_arg=$(test -z "$quick_build" && echo '--release' || echo -n '') 

cargo test --all $build_arg --target $target
cargo build $build_arg --target $target
./snapshot-tests/run.sh $(test -n "$quick_build" && echo '-d' || echo -n '') $build_path/sdoc

if [[ -z "$quick_build" ]];
then
  mkdir -p dist
  declare build=${TRAVIS_TAG:-SNAPSHOT}
  tar czf dist/sdoc-$build-$target.tar.gz -C $build_path sdoc
fi

exit 0
