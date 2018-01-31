help:
	$(info                                                    )
	$(info Available Targets                                  )
	$(info                                                    )
	$(info ---------------------------------------------------)
	$(info Build                                              )
	$(info ---------------------------------------------------)
	$(info build          - A full release build              )
	$(info dev-build      - Faster build for development      )
	$(info clean          - Remove temporary build artifacts  )
	$(info                                                    )
	$(info ---------------------------------------------------)
	$(info Release                                            )
	$(info ---------------------------------------------------)
	$(info tag-release    - Create a tag for the new release  )
	$(info cargo-release  - Create a tag for the new release  )
	$(info                                                    )

build:
	./bin/build.sh

dev-build:
	./bin/build.sh -q

tag-release: VERSION
	./bin/tag-release.sh $$(cat VERSION)

cargo-release:
	./bin/cargo-release.sh

clean:
	cargo clean && rm -rf dist
