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
        let _repo = repo.node.as_ref().unwrap();
        return Repo::new(
            _repo.name.clone(),
            org,
            _repo.created_at.parse().expect("time did not match"),
            Utc::now(),
        );
    }
}

impl RepoQuery {
    pub fn new(
        name: String,
        org: String,
        createdAt: DateTime<Utc>,
        updatedAt: DateTime<Utc>,
        lastrun: DateTime<Utc>,
        topics: Vec<String>,
        languages: Vec<String>,
        stars: i64,
    ) -> RepoQuery {
        return RepoQuery {
            name,
            org,
            createdAt,
            updatedAt,
            lastrun,
            topics,
            languages,
            stars,
        };
    }
    pub fn repoQuery_from_repo(
        repo: &OrgViewOrganizationRepositoriesEdges,
        org: String,
    ) -> RepoQuery {
        let _repo = repo.node.as_ref().unwrap();
        return RepoQuery::new(
            _repo.name.clone(),
            org,
            _repo.created_at.parse().expect("time did not match"),
            _repo.updated_at.parse().expect("time did not match"),
            Utc::now(),
            _repo
                .repository_topics
                .edges
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|r| {
                    r.as_ref()
                        .unwrap()
                        .node
                        .as_ref()
                        .unwrap()
                        .topic
                        .name
                        .clone()
                })
                .collect(),
            // _repo.node.as_ref().unwrap().topics,
            _repo
                .languages
                .as_ref()
                .unwrap()
                .edges
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|r| r.as_ref().unwrap().node.name.clone())
                .collect(),
            _repo.stargazers.total_count,
        );
    }
}
