#! /bin/bash

BLUE="$(tput setaf 4)"
WHITE="$(tput setaf 7)"
YELLOW="$(tput setaf 3)"
GREEN="$(tput setaf 2)"
RED="$(tput setaf 1)"

while getopts ":d" opt; do
  case "$opt" in
    d) dev='true';
  esac
done
shift $((OPTIND-1))

exe=${1:?Executable expected}

test_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
export COMMANDS_DIRECTORY="$test_dir/data"
export CLI_NAME=sdoc

echo
echo "${BLUE}Testing$WHITE"
echo $exe 
echo "${BLUE}in$WHITE"
echo $COMMANDS_DIRECTORY
echo

function add_snapshot() {
	content=${1:?Content expected}
	filename=${2:?Filename expected}
	echo -n 'Approve? (y/n) '
	read approve
	[[ "$approve" == "y" ]] && echo -n "$actual" > $filename
}

function should () {
  name=${1:?Name expected}
  command=${2:?Command expected}
  expected_exit_code=${3:-0}

  echo -n "$name ($2): "
  actual=$(bash -c "$2")
  actual_exit_code=$?
  filename="$test_dir/snapshots/$(tr ' ' '_' <<< $name).snap"

  if [[ "$expected_exit_code" != "$actual_exit_code" ]];
  then
    echo "${RED}FAIL$WHITE"
    echo
    echo "Expected exit code $expected_exit_code but was $actual_exit_code"
    exit 1 
  fi

  if [[ ! -f "$filename" ]];
  then
	echo
	echo
	echo 'New snapshot found!'
	echo
	echo '************************************'
	echo "$actual"
	echo '************************************'
	echo
	add_snapshot "$actual" "$filename"
  else
	test_output=$(echo -n "$actual" | diff -y $filename - )
	if [[ $? > 0 ]];
	then
	  echo "${RED}fail$WHITE"
	  echo
	  echo "$test_output"

	  if [[ "$dev" == "true" ]];
	  then
		  add_snapshot "$actual" "$filename"
	  else
		  exit 1
  	  fi
	else
          echo "${GREEN}OK${WHITE}"	
	fi
  fi

}

