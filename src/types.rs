use crate::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges;
use std::time::SystemTime;

#[derive(Eq, PartialEq, Debug)]
pub struct Org {
    name: String,
    lastrun: SystemTime,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Repo {
    pub name: String,
    pub org: String,
    pub createdAt: SystemTime,
    pub lastrun: SystemTime,
}

#[derive(Eq, PartialEq, Debug)]
pub struct RepoQuery {
    name: String,
    org: String,
    createdAt: SystemTime,
    updatedAt: SystemTime,
    lastrun: SystemTime,
    topics: Vec<String>,
    languages: Vec<String>,
    stars: i64,
}

impl Repo {
    pub fn repo_from_repo(repo: &OrgViewOrganizationRepositoriesEdges) -> Repo {
        return Repo {
            name: "Repo".to_string(),
            org: "Google".to_string(),
            createdAt: std::time::SystemTime::now(),
            lastrun: std::time::SystemTime::now(),
        };
    }
}
