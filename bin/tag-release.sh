#! /bin/bash

set -eu

version=${1:?First argument is the version you want to tag, like '1.0.0'}

sed -i "" -E "s/^version = \".*\"/version = \"$version\"/" Cargo.toml

git commit -am "Release v$version"
git tag "v$version"

exit 0
