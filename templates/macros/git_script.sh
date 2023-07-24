{% macro git_script(gitActionLabel, gitAction, groupNode) -%}
#!/bin/bash
# This file is generated, do not update manually
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ME="$(basename "$0")"

pushd "$DIR" || exit 1

echo "git {{ gitActionLabel }} - group {{ groupNode.group.full_path }}"

{%- set projectsLength = groupNode.group.projects | length %}
{% if projectsLength > 0 %}
echo "git {{ gitActionLabel }} {{ projectsLength }} projects of {{ groupNode.group.full_path }}"
  {%- for project in groupNode.group.projects %}
echo "git {{ gitActionLabel }} {{ project.path_with_namespace }}"
git {{ gitAction }} {{ project.ssh_url_to_repo }}
  {%- endfor %}
{%- endif  %}

{%- set childrenLength = groupNode.children | length %}
{% if childrenLength > 0 %}
echo "git {{ gitActionLabel }} {{ childrenLength }} sub-groups of {{ groupNode.group.full_path }}"
  {%- for subGroup in groupNode.children %}
"{{ subGroup.group.path }}/$ME"
  {%- endfor  %}
{%- endif %}

popd || exit 1
{% endmacro git_script %}