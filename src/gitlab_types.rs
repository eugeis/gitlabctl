use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DefaultOnError;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GroupIdName {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CondensedGroupSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleGroupSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_path: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GroupSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub visibility: String, // 'public' | 'internal' | 'private'
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub share_with_group_lock: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub require_two_factor_authentication: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub two_factor_grace_period: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub project_creation_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub auto_devops_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub subgroup_creation_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub emails_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mentions_disabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub lfs_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub default_branch_protection: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub request_access_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub parent_id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ldap_cn: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ldap_access: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub marked_for_deletion_on: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub membership_lock: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SharedWithGroup {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_full_path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_access_level: u32,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name_with_namespace: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path_with_namespace: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub default_branch: String,
    #[serde_as(as = "Vec<DefaultOnError>")]
    pub topics: Vec<String>,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ssh_url_to_repo: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub http_url_to_repo: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub readme_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub forks_count: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub star_count: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_activity_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub namespace: CondensedNamespaceSchema,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description_html: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub visibility: String, // 'public' | 'internal' | 'private'
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_requests_template: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub empty_repo: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub issues_template: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub owner: SimpleUserSchemaIdNameCreatedAt,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub issues_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub open_issues_count: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_requests_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub jobs_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub wiki_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub snippets_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub can_create_merge_request_in: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub resolve_outdated_diff_discussions: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub container_registry_access_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub security_and_compliance_access_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub container_expiration_policy: ContainerExpirationPolicy,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub updated_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub creator_id: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub import_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub import_type: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub import_status: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub import_error: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub permissions: Permissions,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub archived: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub license_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub license: ProjectLicenseSchema,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub shared_runners_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_runners_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub runners_token: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_default_git_depth: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_forward_deployment_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_forward_deployment_rollback_allowed: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_allow_fork_pipelines_to_run_in_parent_project: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_separated_caches: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ci_restrict_pipeline_cancellation_role: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub public_jobs: bool,
    #[serde_as(as = "Vec<DefaultOnError>")]
    pub shared_with_groups: Vec<SharedWithGroup>,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub repository_storage: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub only_allow_merge_if_pipeline_succeeds: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub allow_merge_on_skipped_pipeline: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub restrict_user_defined_variables: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub only_allow_merge_if_all_discussions_are_resolved: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub remove_source_branch_after_merge: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub printing_merge_requests_link_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub request_access_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_method: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub squash_option: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub auto_devops_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub auto_devops_deploy_strategy: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mirror: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mirror_user_id: i32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mirror_trigger_builds: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub only_mirror_protected_branches: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mirror_overwrites_diverged_branches: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub external_authorization_classification_label: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub packages_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub service_desk_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub service_desk_address: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub autoclose_referenced_issues: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub suggestion_commit_message: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub enforce_auth_checks_on_uploads: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_commit_template: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub squash_commit_template: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub issue_branch_template: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub marked_for_deletion_on: String,
    #[serde_as(as = "Vec<_>")]
    #[serde(default)]
    pub compliance_frameworks: Vec<String>,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub warn_about_potentially_unwanted_characters: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub container_registry_image_prefix: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub _links: Links,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExpandedGroupSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub visibility: String, // 'public' | 'internal' | 'private'
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub share_with_group_lock: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub require_two_factor_authentication: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub two_factor_grace_period: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub project_creation_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub auto_devops_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub subgroup_creation_level: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub emails_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub mentions_disabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub lfs_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub default_branch_protection: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub request_access_enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub parent_id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ldap_cn: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ldap_access: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub marked_for_deletion_on: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub membership_lock: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub runners_token: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub file_template_project_id: u32,
    #[serde_as(as = "Vec<_>")]
    pub shared_with_groups: Vec<SharedWithGroup>,
    #[serde_as(as = "Vec<_>")]
    pub projects: Vec<ProjectSchema>,
    #[serde_as(as = "Vec<_>")]
    pub shared_projects: Vec<ProjectSchema>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CondensedProjectSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleProjectSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name_with_namespace: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path_with_namespace: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub default_branch: String,
    #[serde_as(as = "Vec<DefaultOnError>")]
    pub topics: Vec<String>,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ssh_url_to_repo: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub http_url_to_repo: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub readme_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub forks_count: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub star_count: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_activity_at: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub namespace: CondensedNamespaceSchema,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleUserSchemaIdNameCreatedAt {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerExpirationPolicy {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub cadence: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub enabled: bool,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub keep_n: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub older_than: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name_regex_delete: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name_regex_keep: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub next_run_at: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Permissions {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub project_access: AccessLevel,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_access: AccessLevel,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessLevel {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub access_level: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub notification_level: u32,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Links {
    #[serde(rename = "self")]
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub self_link: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub issues: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_requests: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub repo_branches: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub labels: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub events: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub members: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub cluster_agents: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CondensedNamespaceSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub kind: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub parent_id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NamespaceSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub kind: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub full_path: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub parent_id: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub members_count_with_descendants: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub billable_members_count: u32,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub plan: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub trial_ends_on: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub trial: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectLicenseSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub key: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub nickname: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub html_url: String,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub source_url: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PipelineSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub iid: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub project_id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub sha: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub ref_: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub status: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub source: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub updated_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExpandedPipelineSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub before_sha: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub tag: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub yaml_errors: Option<serde_json::Value>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub user: SimpleUserSchema,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub started_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub finished_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub committed_at: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub duration: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub queued_duration: Option<serde_json::Value>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub coverage: Option<serde_json::Value>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub detailed_status: DetailedStatus,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PipelineTestCaseSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub status: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub classname: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub execution_time: f64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub system_output: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub stack_trace: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PipelineTestSuiteSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub total_time: f64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub total_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub success_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub failed_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skipped_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub error_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub test_cases: Option<Vec<PipelineTestCaseSchema>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PipelineTestReportSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub total_time: f64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub total_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub success_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub failed_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skipped_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub error_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub test_suites: Option<Vec<PipelineTestSuiteSchema>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PipelineTestReportSummarySchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub total: SummaryTotal,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub test_suites: Option<Vec<PipelineTestSuiteSchema>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SummaryTotal {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub time: f64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub success: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub failed: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skipped: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub error: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub suite_error: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DetailedStatus {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub icon: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub text: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub label: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub tooltip: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub has_details: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub details_path: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub illustration: Option<serde_json::Value>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub favicon: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SimpleUserSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub username: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub state: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub avatar_url: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub web_url: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_at: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub locked: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub bio: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub bot: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub location: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub public_email: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skype: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub linkedin: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub twitter: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub discord: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub website_url: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub pronouns: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub organization: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub job_title: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub work_information: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub followers: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub following: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub local_time: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub is_followed: Option<bool>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExpandedUserSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub is_admin: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub bot: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_sign_in_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub confirmed_at: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_activity_on: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub email: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub theme_id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub color_scheme_id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub projects_limit: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub current_sign_in_at: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub note: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub identities: Option<Vec<Identity>>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub can_create_group: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub can_create_project: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub two_factor_enabled: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub external: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub private_profile: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub namespace_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_by: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdminUserSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub current_sign_in_ip: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_sign_in_ip: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub using_license_seat: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub email_reset_offered_at: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub shared_runners_minutes_limit: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub extra_shared_runners_minutes_limit: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub is_auditor: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub provisioned_by_group_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub plan: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub trial: Option<bool>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserActivitySchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub username: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_activity_on: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub last_activity_at: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserStatusSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub emoji: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub availability: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub message: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub message_html: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub clear_status_at: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserPreferenceSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub user_id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub view_diffs_file_by_file: bool,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub show_whitespace_in_diffs: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserCountSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_requests: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub assigned_issues: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub assigned_merge_requests: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub review_requested_merge_requests: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub todos: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserAssociationCountSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub groups_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub projects_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub issues_count: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub merge_requests_count: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserMembershipSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub source_id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub source_name: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub source_type: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub access_level: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserRunnerSchema {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id: u64,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub token: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub token_expires_at: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Identity {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub provider: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub extern_uid: String,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub saml_provider_id: u64,
}

// Define the options types using structs and serde_as for the optional fields

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AllUsersOptions {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub order_by: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_by: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub sort: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub two_factor: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub without_projects: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub admins: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub saml_provider_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skip_ldap: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub search: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub username: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub active: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub blocked: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub external: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub exclude_internal: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub exclude_external: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub without_project_bots: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_before: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub created_after: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub with_custom_attributes: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub custom_attributes: Option<std::collections::HashMap<String, String>>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub provider: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub extern_uid: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateUserOptions {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub admin: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub auditor: Option<bool>,

    //#[serde_as(as = "DefaultOnError")]
    //#[serde(default)]
    //pub avatar: Option<Avatar>,
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub bio: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub can_create_group: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub color_scheme_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub commit_email: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub email: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub extern_uid: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub external: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub extra_shared_runners_minutes_limit: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub force_random_password: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_id_for_saml: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub linkedin: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub location: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub name: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub note: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub organization: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub password: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub private_profile: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub projects_limit: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub pronouns: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub provider: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub public_email: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub reset_password: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub shared_runners_minutes_limit: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skip_confirmation: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub skype: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub theme_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub twitter: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub discord: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub username: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub view_diffs_file_by_file: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub website_url: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateUserCIRunnerOptions {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub group_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub project_id: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub description: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub paused: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub locked: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub run_untagged: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub tag_list: Option<Vec<String>>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub access_level: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub maximum_timeout: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub maintenance_note: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AllUserProjectsOptions {
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub archived: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id_after: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub id_before: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub membership: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub min_access_level: Option<u64>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub order_by: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub owned: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub search: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub simple: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub sort: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub starred: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub statistics: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub visibility: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub with_custom_attributes: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub with_issues_enabled: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub with_merge_requests_enabled: Option<bool>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub with_programming_language: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub updated_before: Option<String>,

    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub updated_after: Option<String>,
}
