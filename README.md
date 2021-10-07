# sdoc 

[![Build Status](https://api.travis-ci.com/matthewwoodruff/sdoc.svg?branch=master)](https://travis-ci.org/matthewwoodruff/sdoc) [![dependency status](https://deps.rs/repo/github/matthewwoodruff/sdoc/status.svg)](https://deps.rs/repo/github/matthewwoodruff/sdoc) [![](https://img.shields.io/crates/v/sdoc.svg)](https://crates.io/crates/sdoc)


A framework for building custom CLIs around shell and executables. Commands are defined in yaml and can be sub-commands, shell, or an executable. By convention your root yaml file lives in a directory named the same as your cli name and sub-command configuration in sub directories following the same pattern.

## Install

##### Homebrew

```
brew tap matthewwoodruff/sdoc https://github.com/matthewwoodruff/sdoc
brew install sdoc
```

##### Cargo

```
cargo install sdoc
```

##### Manual

You can download the binary from [GitHub Releases](https://github.com/matthewwoodruff/sdoc/releases) and add to your `$PATH`


## Create CLI


- Create a new directory for your CLI `mkdir <cli-name>`
- Execute `sdoc init` and follow the prompts
- Your CLI will be available by executing `./bin/<cli-name>`
- Modify `./<my-cli>/commands.yaml` to add custom commands and sub-commands


##### Example

see https://github.com/matthewwoodruff/sdoc/tree/master/tests/data/example-cli for an example cli configuration.


## Yaml Configuration

##### Command

| Field        | Type               | Required   | Description
|:-------------|:-------------------|:-----------|:-----------
| name         | string             | yes        | Command name
| description  | string             | yes        | Description that will be show in help 
| type         | type               | yes        | Type of command. See below for options.
| alias        | string             | no         | A shorter version of the command name
| usage        | string             | no         | Usage text to be shown in help
| dependencies | list\<dependency\> | no         | List of dependencies. See below for options.
| min_args     | int                | no         | Minimum amount of arguments to be supplied

##### Type

| Option       | Value   | Description
|:-------------|:--------|:-----------
| node         | -       | Sub-command 
| shell        | string  | Shell code to execute
| script       | string  | A script file relative to the commands file

##### Dependencies

| Field        | Type               | Required | Description
|:-------------|:-------------------|:---------|:-----------
| value        | dependency-type    | yes      | Dependency type. See below for options.
| description  | string             | yes      | Shown against dependency in command usage

##### Dependency Type

| Option       | Value    | Description
|:-------------|:---------|:-----------
| command      | string   | An executable in your $PATH that must exist
| envar        | string   | An environment variable that must be set


