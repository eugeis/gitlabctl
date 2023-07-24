{% import "macros/git_script_repo.sh" as macros %}
{{ macros::git_script_repo(gitActionLabel="pull", gitAction="pull", groupNode="groupNode") }}