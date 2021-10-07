#! /bin/bash

set -eu

echo Enter version. Current is $(cat VERSION)
read version

version_tag="v$version"

if git rev-parse "$version_tag" >/dev/null 2>&1
then
  echo "Version $version_tag already exists"
  exit 1
fi

echo $version > VERSION

sed -i "" -E "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
#sed -i "" -E "s/ version \'.*\'/version \'$version\'/" pkg/brew/sdoc.rb

cargo check

git reset .
git add Cargo.toml Cargo.lock pkg/brew/sdoc.rb VERSION
git commit -S -m "Release $version_tag"
git tag -s "$version_tag" -m "Release $version_tag"

git push
git push --tags

exit 0
