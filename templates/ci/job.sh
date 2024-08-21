#!/bin/bash
# This file is generated, do not update manually

{%- if not is_template %}
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ME="$(basename "$0")"

pushd "$DIR" || exit 1

source .env.sh
{% endif %}

{% if variables -%}
# Variables
{%- for variable in variables %}
export {{ variable.name }}="{{ variable.value }}"
{%- endfor %}
{%- endif %}

{% if job.extends -%}
# Extends
{%- if job.extends is iterable %}
{%- for parent in job.extends %}
source {{ parent }}.sh
{% endfor %}
{% else %}
source {{ job.extends }}.sh
{%- endif %}
{%- endif %}

{%- if job.before_script %}
# Before Script
{%- for command in job.before_script %}
{{ command }}
{%- endfor %}
{%- endif %}

{%- if job.script %}
# Script
{%- for command in job.script %}
{{ command }}
{%- endfor %}
{%- endif %}

{% if not is_template %}
popd || exit 1
{%- endif %}