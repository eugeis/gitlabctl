{% import "git/macros/git_script_repo_recursive.sh" as macros %}
{{ macros::git_script_repo_recursive(gitActionLabel="pull", gitAction="pull", groupNode="groupNode") }}