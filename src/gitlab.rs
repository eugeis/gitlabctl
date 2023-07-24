use serde::{Serialize, Deserialize};

use gitlab::api::{ApiError, groups};
use gitlab::api::groups::subgroups;
use gitlab::{Gitlab, RestError};
use gitlab::api::Query;
use gitlab::types::Group;
use gitlab::types::GroupDetail;

use std::collections::HashSet;

use std::path::Path;
use log::{info, warn};
use crate::common::{Error, Result};
use crate::common::Error::AlreadyHandled;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupNode {
    pub group:            GroupDetail,
    pub children:         Vec<GroupNode>,
    pub relative_root_path: String
}

impl GroupNode {
    fn set_as_relative_root_path(&mut self) {
        self.relative_root_path = self.group.path.clone();
        self.set_children_relative_root_path()
    }

    fn set_children_relative_root_path(&mut self) {
        for mut child in self.children.iter_mut() {
            child.relative_root_path =
                Path::new(&self.relative_root_path).
                    join(&child.group.path).into_os_string().into_string().unwrap();
            if !child.children.is_empty() {
                child.set_children_relative_root_path()
            }
        }
    }

    fn on_child(&mut self, child: GroupNode) {
        self.children.push(child)
    }
}

struct GroupIdName {
    id: u64,
    name: String
}

pub struct GroupNodeReader {
    gitlab: Gitlab,
    ignore_group_names: HashSet<String>,
    already_handled_groups: HashSet<u64>,
}

impl GroupNodeReader {
    pub fn new<H, T>(gitlab_url: H, gitlab_token: T) -> Result<GroupNodeReader>
        where
            H: AsRef<str>,
            T: Into<String>,
    {
        match Gitlab::new(gitlab_url, gitlab_token) {
            Ok(gitlab) => Ok(GroupNodeReader {
                gitlab,
                ignore_group_names: Default::default(),
                already_handled_groups: Default::default(),
            }),
            Err(err) => Err(Error::GitlabError{source: err})
        }
    }

    pub fn read(&mut self, group_name_or_id: &str) -> Result<GroupNode> {

        let group_detail: Result<GroupDetail, ApiError<RestError>> = groups::Group::builder()
            .group(group_name_or_id)
            .build().unwrap().query(&self.gitlab);

        return match group_detail {
            Ok(group) => {
                let mut r = self.read_for_group_detail(group);
                if r.is_ok() {
                    let node = r.as_mut().unwrap();
                    node.set_as_relative_root_path()
                }
                r
            },
            Err(err) => {
                Err(Error::General {source: err})
            }
        }
    }

    fn mark_as_already_handled(&mut self, group_id: u64) {
        self.already_handled_groups.insert(group_id);
    }

    fn read_for_group_detail(&mut self, group: GroupDetail) -> Result<GroupNode> {
        self.mark_as_already_handled(group.id.value());

        let group_id = &group.id.value();

        println!("handle {}({})", group_id, &group.name);

        let mut group_node = GroupNode{
            group,
            children: vec![],
            relative_root_path: "".to_string(),
        };

        let group_id_names = group_node.group.projects.iter()
            .flat_map(|prj| prj.shared_with_groups.iter()
                .map(|shared_group|GroupIdName{id:shared_group.group_id.value(), name: shared_group.group_name.clone()})).collect::<Vec<GroupIdName>>();

        for group_id_name in &group_id_names {
            match self.read_child_group(&group_id_name.id, &group_id_name.name) {
                Ok(node) => group_node.on_child(node),
                Err(error) => match &error {
                    AlreadyHandled => {},
                    other_error => eprintln!("reading child group: {}", other_error),
                }
            };
        }

        self.read_sub_groups(group_id, |node: GroupNode| {
            group_node.on_child(node)
        });

        Ok(group_node)
    }

    fn read_child_group(&mut self, group_id: &u64, group_name: &str) -> Result<GroupNode> {
        if !self.shall_read_group(group_id, group_name) {
            return Err(Error::AlreadyHandled)
        }

        let group_detail: GroupDetail = groups::Group::builder()
            .group(group_id.clone())
            .build().unwrap().query(&self.gitlab)?;

        self.read_for_group_detail(group_detail)
    }

    fn read_sub_groups(&mut self, group_id: &u64, mut on_child: impl FnMut(GroupNode)) {
        let sub_groups: Vec<Group> = subgroups::GroupSubgroups::builder()
            .group(group_id.clone())
            .build().unwrap().query(&self.gitlab).unwrap();

        for sub_group in sub_groups {
            match self.read_child_group( &sub_group.id.value(), &sub_group.name) {
                Ok(group_node) => on_child(group_node),
                Err(err) => eprintln!("can't read child group: {:?}", err)
            }
        }
    }

    fn shall_read_group(&mut self, group_id: &u64, group_name: &str) -> bool {
        let ret = !self.already_handled_groups.contains(&group_id) &&
            !self.ignore_group_names.contains(group_name);
        if ret {
            self.mark_as_already_handled(group_id.clone())
        }
        ret
    }

}