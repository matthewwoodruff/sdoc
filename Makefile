help:
	$(info Available Targets                                  )
	$(info ---------------------------------------------------)
	$(info build          - A full release build              )
	$(info dev-build      - Faster build for development      )
	$(info tag-release    - Create a tag for the new release  )
	$(info cargo-release  - Create a tag for the new release  )

build:
	./build.sh

dev-build:
	./build.sh -q

tag-release: VERSION
	./bin/tag-release.sh $$(cat VERSION)

cargo-release:
	./bin/cargo-release.sh
