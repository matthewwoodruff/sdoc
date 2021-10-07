help:
	$(info                                                      )
	$(info Available Targets                                    )
	$(info                                                      )
	$(info -----------------------------------------------------)
	$(info Development                                          )
	$(info -----------------------------------------------------)
	$(info build          - A full release build                )
	$(info dev-build      - Faster build for development        )
	$(info clean          - Remove temporary build artifacts    )
	$(info push           - Push code to remote (builds first)  )
	$(info                                                      )
	$(info -----------------------------------------------------)
	$(info Release                                              )
	$(info -----------------------------------------------------)
	$(info tag-release    - Create a tag for the new release    )
	$(info cargo-release  - Create a tag for the new release    )
	$(info                                                      )

.PHONY: build
build: clean
	./bin/build.sh

.PHONY: docker-build
docker-build:
	docker run -w /work -v $$(pwd):/work rust:1.55 make build

.PHONY: dev-build
dev-build:
	./bin/build.sh -q

.PHONY: clean
clean:
	cargo clean && rm -rf dist

.PHONY: push
push: build
	git push origin master

.PHONY: tag-release
tag-release: VERSION
	./bin/tag-release.sh $$(cat VERSION)

.PHONY: cargo-release
cargo-release:
	./bin/cargo-release.sh