#! /usr/bin/env bash

echo "Create tar file for $TRAVIS_OS_NAME"

set +ex

main() {
    local src=$(pwd) 
    local stage=$(mktemp -d)

    cp target/$TARGET/release/sdoc $stage/

    cd $stage
    tar czf $src/sdoc-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main

