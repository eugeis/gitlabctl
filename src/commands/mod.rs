use clap::Subcommand;

pub mod generate_git_scripts;
pub mod show_pipelines;
pub mod delete_pipelines;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate Git scripts
    GenerateGitScripts(generate_git_scripts::GenerateGitScriptsCommand),

    /// Show Pipelines
    ShowPipelines(show_pipelines::ShowPipelinesCommand),

    /// Delete Pipelines
    DeletePipelines(delete_pipelines::DeletePipelinesCommand),
}
