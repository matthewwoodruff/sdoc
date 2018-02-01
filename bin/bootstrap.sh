#! /bin/sh
set -ue

WHITE="$(tput setaf 7 2> /dev/null || echo -n '')"
YELLOW="$(tput setaf 3 2> /dev/null || echo -n '')"
BLUE="$(tput setaf 4 2> /dev/null || echo -n '')"
GREEN="$(tput setaf 2 2> /dev/null || echo -n '')"
RED="$(tput setaf 1 2> /dev/null || echo -n '')"

VERSION=0.2.2

main() {
  echo
  echo "${BLUE}Welcome to the sdoc initialiser$WHITE"

  setup_directory="$(pwd)"
  cli_name=$(required 'CLI name (required)(no special characters):' | tr ' ' '-')
  setup_git_repo=$(yes_or_no 'Setup git repo? (yes/no):' "yes no")
  go_ahead=$(yes_or_no "Create cli ${GREEN}$cli_name${WHITE} in ${GREEN}$setup_directory${WHITE} with git repo ${GREEN}$setup_git_repo${WHITE}. Ok? (yes/no):" "yes no")

  echo
  if [ "$go_ahead" = 'no' ]
  then
    echo "${RED}Setup aborted$WHITE"
    exit 0
  fi

  new_bin=$(cat <<HERE
#! /bin/bash
set -ue

cd \$( dirname "\${BASH_SOURCE[0]}" )

if ! [ -e "bin/sdoc" ]
then
  echo "Downloading sdoc"
  target=\$([ "\$(uname -s)" = 'Darwin' ] && echo 'apple-darwin' || echo 'unknown-linux-musl')
  mkdir -p bin
  cd bin
  curl -LO --progress-bar https://github.com/matthewwoodruff/sdoc/releases/download/v$VERSION/sdoc-v$VERSION-x86_64-\$target.tar.gz
  tar -xzf *.tar.gz
  rm *.tar.gz
  cd ../
fi

COMMANDS_DIRECTORY=../ CLI_NAME=$cli_name bin/sdoc "\$@"
HERE
)

  echo "${GREEN}1.${WHITE} Creating executable ./bin/$cli_name"
  mkdir -p "$setup_directory/bin"
  echo "$new_bin" > bin/"$cli_name"
  chmod +x bin/"$cli_name"

  commands_yaml=$(cat <<HERE
---
- heading: "First Heading"
  commands:
    - name: hello
      description: Prints hello world
      type:
        Shell: echo hello world
HERE
)

  echo "${GREEN}2.${WHITE} Creating config file ./$cli_name/commands.yaml"
  mkdir -p "$cli_name"
  echo "$commands_yaml" > "$cli_name"/commands.yaml

  if [ "$setup_git_repo" = 'yes' ]
  then
    echo "${GREEN}3.${WHITE} Creating git repo with initial commit"
    git init
    echo 'bin/bin' > .gitignore
    git add bin "$cli_name" .gitignore
    git commit -m "Initial commit"
  fi

  echo
  echo "${GREEN}Setup complete$WHITE"
  echo "Execute ./bin/$cli_name to begin. Even better add '$setup_directory/bin' to your \$PATH"
}

required() {
  name=${1:?Expected name}
  temp_var=''
  while [ -z "$temp_var" ]
  do
    echo "$name " >&2
    read temp_var
  done
  echo "$temp_var"
}

yes_or_no() {
  name=${1:?Expected name}
  temp_var=''
  while ! [ "$temp_var" = 'yes' ] && ! [ "$temp_var" = 'no' ]
  do
    temp_var=$(required "$name")
  done
  echo "$temp_var"
}

main
