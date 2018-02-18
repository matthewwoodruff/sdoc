# sdoc 

[![Build Status](https://travis-ci.org/matthewwoodruff/sdoc.svg?branch=master)](https://travis-ci.org/matthewwoodruff/sdoc) [![dependency status](https://deps.rs/repo/github/matthewwoodruff/sdoc/status.svg)](https://deps.rs/repo/github/matthewwoodruff/sdoc) [![](https://img.shields.io/crates/v/sdoc.svg)](https://crates.io/crates/sdoc)


A framework for building custom CLIs around shell and executables. Commands are defined in yaml and can be sub-commands, shell, or an executable. By convention your root yaml file lives in a directory named the same as your cli name and sub-command configuration in sub directories following the same pattern.


**Directory Structure**

Structure for command `my-cli` with sub commands `build` and `stack`

```
.
└── my-cli
    ├── build
    │   ├── all.sh
    │   └── commands.yaml
    ├── commands.yaml
    ├── update.sh
    └── stack
        ├── commands.yaml
        └── up.sh
```

**YAML Examples**

```

```

## Installation

### Bootstrap

- Follow the bootstrap instructions [here](https://github.com/matthewwoodruff/sdoc-bootstrap)


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

## Yaml Configuration

##### Command

| Field        | Type               | Required   | Description
|:-------------|:-------------------|:-----------|:-----------
| name         | string             | yes        | Command name
| description  | string             | yes        | Description that will be show in help 
| alias        | string             | no         | A shorter version of the command name
| usage        | string             | no         | Usage text to be shown in help
| type         | type               | yes        | Type of command. See below for options.
| dependencies | list\<dependency\> | no         | List of dependencies. See below for options.
| min_args     | int                | no         | Minimum amount of arguments to be supplied


##### Type

| Option       | Value   | Description
|:-------------|:--------|:-----------
| Shell        | string  | Shell code to execute
| Script       | string  | A script file relative to the commands file


##### Dependencies

| Field        | Type               | Required | Description
|:-------------|:-------------------|:---------|:-----------
| value        | dependency-type    | yes      |Dependency type. See below for options.
| description  | string             | yes      | Shown against dependency in command usage

##### Dependency Type

| Option       | Value    | Description
|:-------------|:---------|:-----------
| Command      | string   | An executable in your $PATH that must exist
| Envar        | string   | An environment variable that must be set


