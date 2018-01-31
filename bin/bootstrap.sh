#! /bin/bash
set -e

WHITE="$(tput setaf 7 || echo -n '')"
YELLOW="$(tput setaf 3 || echo -n '')"
BLUE="$(tput setaf 4 || echo -n '')"
GREEN="$(tput setaf 2 || echo -n '')"
RED="$(tput setaf 1 || echo -n '')"

VERSION=0.2.2

echo "${BLUE}Welcome to the sdoc initialiser$WHITE"

function required() {
  name=${1:?Expected name}
  response_options=$2
  while [[ -z "$temp_var" ]] || ([[ -n "$response_options" ]] && ! [[ " $response_options " =~ " $temp_var " ]])
  do
    echo -n "$name " >&2
    read temp_var
  done
  echo "$temp_var"
}

setup_directory="$(pwd)"
cli_name=$(required 'CLI name (required)(no special characters):' | tr ' ' '-')
setup_git_repo=$(required 'Setup git repo? (yes/no):' "yes no") 
go_ahead=$(required "Create cli ${GREEN}$cli_name${WHITE} in ${GREEN}$setup_directory${WHITE} with git repo ${GREEN}$setup_git_repo${WHITE}. Ok? (yes/no):" "yes no")

if [[ "$go_ahead" = 'no' ]]
then
  echo "${RED}Setup aborted$WHITE"
  exit 0
fi
echo

mkdir -p $setup_directory/bin

new_bin=$(cat <<HERE
#! /bin/bash
set -e

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

echo "$new_bin" > bin/"$cli_name"
chmod +x bin/"$cli_name"

mkdir -p "$cli_name"
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
echo "$commands_yaml" > "$cli_name"/commands.yaml

if [ "$setup_git_repo" = 'yes' ]
then
  git init
  echo 'bin/bin' > .gitignore
  git add bin "$cli_name" .gitignore
  git commit -m "Initial commit"
fi

echo
echo "${GREEN}Setup complete$WHITE"
echo "Execute ./bin/$cli_name to begin. Even better add '$setup_directory/bin' to your \$PATH"

exit 0
