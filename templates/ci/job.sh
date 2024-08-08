#!/bin/bash
# This file is generated, do not update manually

# Job: {{ job_name }}
{% if not is_template %}
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ME="$(basename "$0")"

pushd "$DIR" || exit 1
{% endif %}

{% if job.variables -%}
# Variables
{% for var_name, var_value in job.variables %}
export {{ var_name }}="{{ var_value }}"
{% endfor %}
{% endif %}

{% if job.extends %}
# Extends
{% if job.extends is iterable -%}
{% for parent in job.extends -%}
source {{ parent }}.sh
{% endfor %}
{% else %}
source {{ job.extends }}.sh
{% endif %}
{% endif %}

{% if job.before_script -%}
# Before Script
{% for command in job.before_script -%}
{{ command }}
{% endfor %}
{% endif %}

{% if job.script -%}
# Script
{% for command in job.script %}
{{ command }}
{% endfor %}
{% endif %}

{% if not is_template -%}
popd || exit 1
{% endif %}