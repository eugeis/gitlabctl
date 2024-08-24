use crate::gitlab_types::PipelineSchema;
use ::gitlab::api::projects::pipelines::{DeletePipeline, Pipelines};
use ::gitlab::{api, Gitlab};
use clap::Args;
use gitlab::api::Query;

#[derive(Args, Debug)]
pub struct DeletePipelinesCommand {
    /// Project path to delete pipelines from
    #[arg(short = 'p', long)]
    pub project_path: String,
}

impl DeletePipelinesCommand {
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
                    println!("Deleting pipeline {:?}", pipeline.id);

                    let endpoint = DeletePipeline::builder()
                        .project(&self.project_path)
                        .pipeline(pipeline.id)
                        .build()
                        .unwrap();
                    api::ignore(endpoint).query(&gitlab).expect("delete failed");
                }
            }
            Err(err) => println!("Error connecting to GitLab: {:?}", err),
        }
    }
}
