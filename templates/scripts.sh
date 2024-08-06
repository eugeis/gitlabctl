#!/bin/bash
# This file is generated, do not update manually
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ME="$(basename "$0")"

gitlabctl -t "{{ gitlabTokenFile }}" generate-git-scripts --groups "{{ groupNode.group.full_path }}" -o "$DIR/.."