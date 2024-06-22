use serde::{Serialize, Deserialize};
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
