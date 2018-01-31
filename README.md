# sdoc

[![Build Status](https://travis-ci.org/matthewwoodruff/sdoc.svg?branch=master)](https://travis-ci.org/matthewwoodruff/sdoc)

## Getting Started

### Bootstrap

- Create a new directory for your CLI `mkdir my-cli && cd my-cli`
- Execute the bootstrap script and follow the instructions
	`bash -c "$(curl https://raw.githubusercontent.com/matthewwoodruff/sdoc/master/bin/bootstrap.sh -sSf)"`
- Your CLI will be available by running `./bin/my-cli`


### Manual

- Install the sdoc binary
 - If you use cargo then run `cargo install sdoc`
 - Alternatively you can download the binary from [GitHub](https://github.com/matthewwoodruff/sdoc/releases) and add to your `$PATH`
- Create a new directory for your CLI `mkdir my-cli`
- Create a wrapper script that encapsulates configuration for sdoc

 ```
 #! /bin/bash
 COMMANDS_DIRECTORY=/path/to/cli/directory CLI_NAME=my-cli sdoc "$@"
 ```
- Create the top level config file for `my-cli` called `/path/to/cli/directory/my-cli/commands.yaml` 
- Your CLI will be available by running `./my-cli`
