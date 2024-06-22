#!/bin/bash
# This file is generated, do not update manually
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ME="$(basename "$0")"

GITLAB_TOKEN=`cat {{ gitlabTokenFile }}`

gitlabctl --groups "{{ groupNode.group.path }}" -o "$DIR/.." -t "$GITLAB_TOKEN"