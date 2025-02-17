{% import "git/macros/git_script_recursive.sh" as macros %}
{{ macros::git_script_recursive(gitActionLabel="clone", gitAction="clone --recurse-submodules -j8", groupNode="groupNode") }}