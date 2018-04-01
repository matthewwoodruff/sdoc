#! /bin/bash

unset EDITOR

test_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$test_dir/util.sh"

#echo "${BLUE}General$WHITE"
#should 'show command usage when args are insufficient' "$exe min-args" 2
#should 'execute a command with required args' "$exe min-args 'an argument'"
#should 'show command usage when dependency check fails for envar' "$exe deps" 3
#should 'execute command when dependencies are satisfied with envar' "A_NAME=Steve $exe deps"
#should 'show command usage when dependency check fails for command' "$exe com-dep" 3
#should 'execute command when dependencies are satisfied with command' "PATH=\"$PATH:$test_dir/../tests/data/test-bin\" $exe com-dep"

exit 0
