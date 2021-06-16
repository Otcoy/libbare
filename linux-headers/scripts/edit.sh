#!/usr/bin/env bash

pushd work
git rebase --interactive upstream/upstream
popd
