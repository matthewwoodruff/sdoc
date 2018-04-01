#! /bin/bash

unset EDITOR

test_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$test_dir/util.sh"

#echo "${BLUE}Node Command$WHITE"
#should 'show help message when no arguments are supplied' "$exe"
#should 'show help message when unknown command is given' "$exe unknown-command" 1
#should 'show autocomplete for available commands' "AUTO_COMPLETE=1 $exe"
#should 'execute sub command' "$exe sub print"
#should 'execute command with alias' "$exe p"
#echo

#echo "${BLUE}Help Command$WHITE"
#should 'show help for specific command' "$exe help print"
#should 'show help message when help is ran' "$exe help"
#should 'show help for specific command using alias' "$exe help p"
#should 'show help for help command' "$exe help help"
#should 'show help for view command' "$exe help view"
#should 'show help for edit command' "$exe help edit"
#should 'show help for config command' "$exe help config"
#should 'show autocomplete for help command' "AUTO_COMPLETE=2 $exe help"
#should 'return non-zero exit code when asking for help on a non-existing command' "$exe help unknown-command" 1
#echo

echo "${BLUE}View Command$WHITE"
should 'enable viewing a shell command' "$exe view print"
should 'enable viewing a script command' "$exe view script"
should 'show autocomplete for view command' "AUTO_COMPLETE=2 $exe view"
should 'only allow viewing of script and shell commands' "$exe view edit" 1
should 'return non-zero exit code when viewing non-existing command' "$exe view unknown-command" 1
echo

echo "${BLUE}Edit Command$WHITE"
should 'show autocomplete for edit command' "AUTO_COMPLETE=2 $exe edit"
should 'return non-zero exit code when editing a non-existing command' "EDITOR=vim $exe edit unknown-command" 1
should 'return non-zero exit code when EDITOR environment variable is not set when editing a command' "$exe edit script" 3
should 'only allow editing of script commands' "EDITOR=vim $exe edit print" 1
should 'allow editor to be configured when editing a command' "EDITOR=cat $exe edit script"
echo

echo "${BLUE}Script Command$WHITE"
should 'execute a script command' "$exe script"
should 'pass arguments to script correctly' "$exe script 'one two' 'three'"
echo

echo "${BLUE}Shell Command$WHITE"
should 'execute shell command' "$exe print"
echo

echo "${BLUE}Edit Config Command$WHITE"
should 'not return any suggestions for auto complete' "AUTO_COMPLETE=2 $exe config"
should 'allow editor to be configured when editing config' "EDITOR=cat $exe config"
should 'return non-zero exit code when EDITOR environment variable is not set when editing config' "$exe config" 3

echo "${BLUE}General$WHITE"
should 'show command usage when args are insufficient' "$exe min-args" 2
should 'execute a command with required args' "$exe min-args 'an argument'"
should 'show command usage when dependency check fails for envar' "$exe deps" 3
should 'execute command when dependencies are satisfied with envar' "A_NAME=Steve $exe deps"
should 'show command usage when dependency check fails for command' "$exe com-dep" 3
should 'execute command when dependencies are satisfied with command' "PATH=\"$PATH:$test_dir/../tests/data/test-bin\" $exe com-dep"

exit 0
