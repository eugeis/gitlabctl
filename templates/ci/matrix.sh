#!/bin/bash
# This file is generated, do not update manually

{% if variables -%}
# Variables
{%- for variable in variables %}
export {{ variable.name }}="{{ variable.value }}"
{%- endfor %}
{%- endif %}