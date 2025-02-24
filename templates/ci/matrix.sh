#!/bin/bash
# This file is generated, do not update manually

{%- if not is_template %}
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
WORKING_DIR="$SCRIPT_DIR/.."

pushd "$SCRIPT_DIR" || exit 1

source .env.sh
{% endif %}

{% if variables -%}
# Variables
{%- for variable in variables %}
export {{ variable.name }}="{{ variable.value }}"
{%- endfor %}
{%- endif %}

{% if not is_template %}
popd || exit 1
{%- endif %}