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

build: clean
	./bin/build.sh

docker-build:
	docker run -w /work -v $$(pwd):/work rust:1.25 make build

dev-build:
	./bin/build.sh -q

clean:
	cargo clean && rm -rf dist

push: build
	git push origin master

tag-release: VERSION
	./bin/tag-release.sh $$(cat VERSION)

cargo-release:
	./bin/cargo-release.sh