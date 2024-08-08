#!/bin/bash
# This file is generated, do not update manually

{% if variables -%}
# Variables
{% for var_name, var_value in job.variables -%}
export {{ var_name }}=""
{% endfor -%}
{% endif -%}