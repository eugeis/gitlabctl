{% import "git/macros/git_script_repo.sh" as macros %}
{{ macros::git_script_repo(gitActionLabel="status", gitAction="status", groupNode="groupNode") }}