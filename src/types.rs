use crate::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges;
use chrono::prelude::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Org {
    name: String,
    lastrun: DateTime<Utc>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Repo {
    pub name: String,
    pub org: String,
    pub createdAt: DateTime<Utc>,
    pub lastrun: DateTime<Utc>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct RepoQuery {
    name: String,
    org: String,
    createdAt: DateTime<Utc>,
    updatedAt: DateTime<Utc>,
    lastrun: DateTime<Utc>,
    topics: Vec<String>,
    languages: Vec<String>,
    stars: i64,
}

impl Repo {
    pub fn new(
        name: String,
        org: String,
        createdAt: DateTime<Utc>,
        lastrun: DateTime<Utc>,
    ) -> Repo {
        return Repo {
            name,
            org,
            createdAt,
            lastrun,
        };
    }

    pub fn repo_from_repo(repo: &OrgViewOrganizationRepositoriesEdges, org: String) -> Repo {
        return Repo::new(
            repo.node.as_ref().unwrap().name.clone(),
            org,
            repo.node
                .as_ref()
                .unwrap()
                .created_at
                .parse()
                .expect("time did not match"),
            Utc::now(),
        );
    }
}
