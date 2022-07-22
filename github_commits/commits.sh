#!/bin/bash

# You can run this script from a directory with
# multiple repositories. Update the name and date
# arguments depending on what you're trying to do 

for filename in *; do
  echo $filename

  if [ -d $filename ]; then
    # Is directory
    pushd $filename
    ~/Development/kmadsen/rust/github_commits/target/release/github_commits kyle 2022-01-01
    popd
  fi
done