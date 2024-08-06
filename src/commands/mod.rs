use clap::Subcommand;

pub mod generate_git_scripts;
pub mod show_pipelines;
pub mod delete_pipelines;
pub mod generate_bash_scripts; // New command

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate Git scripts
    GenerateGitScripts(generate_git_scripts::GenerateGitScriptsCommand),

    /// Show Pipelines
    ShowPipelines(show_pipelines::ShowPipelinesCommand),

    /// Delete Pipelines
    DeletePipelines(delete_pipelines::DeletePipelinesCommand),

    /// Generate bash scripts from gitlab-ci.yml
    GenerateBashScripts(generate_bash_scripts::GenerateBashScriptsCommand), // New command
}
