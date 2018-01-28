#! /bin/bash

test_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$test_dir/util.sh"

echo "${BLUE}Should$WHITE"
should 'show help message when no arguments are supplied' "$exe"
should 'show help message when unknown command is given' "$exe unknown-command"
should 'execute shell command' "$exe print"
should 'execute command with alias' "$exe p"
should 'show help for specific command' "$exe help print"
should 'show help for specific command using alias' "$exe help p"
should 'enable viewing a shell command' "$exe view print"
should 'show help for help command' "$exe help help"
should 'show help for view command' "$exe help view"
should 'show help for edit command' "$exe help edit"
should 'show help for edit-config command' "$exe help edit-config"
should 'show autocomplete for available commands' "AUTO_COMPLETE=1 $exe"
should 'show autocomplete for view command' "AUTO_COMPLETE=2 $exe view"
should 'show autocomplete for help command' "AUTO_COMPLETE=2 $exe help"
should 'show autocomplete for edit command' "AUTO_COMPLETE=2 $exe edit"
should 'show command usage when args are insufficient' "$exe min-args" 1
should 'execute a command with required args' "$exe min-args 'an argument'"
should 'show command usage when dependency check fails for envar' "$exe deps" 1
should 'execute command when dependencies are satisfied with envar' "A_NAME=Steve $exe deps"
should 'show command usage when dependency check fails for command' "$exe com-dep" 1
should 'execute command when dependencies are satisfied with command' "PATH=\"$PATH:$test_dir/data/test-bin\" $exe com-dep"
should 'execute a script command' "$exe script"
should 'pass arguments to script correctly' "$exe script 'one two' 'three'"
should 'execute sub command' "$exe sub print"
exit 0
