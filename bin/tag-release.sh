#! /bin/bash

set -eu

version=${1:?First argument is the version you want to tag, like '1.0.0'}
version_tag="v$version"

if git rev-parse "$version_tag" >/dev/null 2>&1
then
  echo "Version $version_tag already exists"
  exit 1
fi

sed -i "" -E "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
#sed -i "" -E "s/ version \'.*\'/version \'$version\'/" pkg/brew/sdoc.rb

cargo check

git reset .
git add Cargo.toml Cargo.lock pkg/brew/sdoc.rb VERSION
git commit -S -m "Release $version_tag"
git tag -s "$version_tag" -m "Release $version_tag"

exit 0
