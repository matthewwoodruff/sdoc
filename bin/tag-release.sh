#! /bin/bash

set -evu

version=${1:?First argument is the version you want to tag, like '1.0.0'}

sed -i "" -E "s/^version = \".*\"/version = \"$version\"/" Cargo.toml

git tag "v$version"

exit 0
