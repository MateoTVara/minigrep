#! /usr/bin/env bash

mkdir -p tmp

cd ~/Projects/rust/minigrep/

git add .
git diff --staged > tmp/staged.diff
git log > tmp/commits.log