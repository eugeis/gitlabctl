{% import "macros/git_script.sh" as macros %}
{{ macros::git_script(gitActionLabel="clone", gitAction="clone --recurse-submodules -j8", groupNode="groupNode") }}