use crate::gitlab_types::PipelineSchema;
use ::gitlab::api::projects::pipelines::Pipelines;
use ::gitlab::Gitlab;
use clap::Args;
use gitlab::api::Query;

#[derive(Args, Debug)]
pub struct ShowPipelinesCommand {
    /// Project path to show pipelines for
    #[arg(short, long)]
    pub project_path: String,
}

impl ShowPipelinesCommand {
    pub fn run(&self, url: &str, gitlab_token: &str) {
        match Gitlab::new(url, gitlab_token) {
            Ok(gitlab) => {
                let pipelines: Vec<PipelineSchema> = Pipelines::builder()
                    .project(&self.project_path)
                    .build()
                    .unwrap()
                    .query(&gitlab)
                    .unwrap();

                for pipeline in pipelines.iter() {
                    println!("Pipeline ID: {:?}", pipeline.id);
                }
            }
            Err(err) => println!("Error connecting to GitLab: {:?}", err),
        }
    }
}
