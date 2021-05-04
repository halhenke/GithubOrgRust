use crate::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use serde_json::{Map, Value};

pub const SQLITE_DB: &'static str = "sqlite://rust-git-org.sqlite";
// pub const ORGS: &'static Vec<&str> = Vec::from([
// static ORGS: Vec<&str> = Vec::from([
pub const ORGS: &[&str] = &[
    "google",
    "google-research",
    "PAIR-code",
    "facebook",
    "microsoft",
    "deepmind",
    "rapidsai",
    "openai",
    "JuliaMath",
    "JuliaData",
    "queryverse",
    "FluxML",
];

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Org {
    pub name: String,
    pub lastrun: DateTime<Utc>,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Repo {
    pub name: String,
    pub org: String,
    pub createdAt: DateTime<Utc>,
    pub lastrun: DateTime<Utc>,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct RepoQuery {
    pub name: String,
    pub org: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub lastrun: DateTime<Utc>,
    pub topics: Vec<String>,
    pub languages: Vec<String>,
    pub stars: i64,
}

impl Org {
    pub fn new(name: String, lastrun: DateTime<Utc>) -> Org {
        return Org { name, lastrun };
    }
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

    pub fn repo_from_repo(
        repo: &OrgViewOrganizationRepositoriesEdges,
        org: String,
        run_time: DateTime<Utc>,
    ) -> Repo {
        let _repo = repo.node.as_ref().unwrap();
        return Repo::new(
            _repo.name.clone(),
            org,
            _repo.created_at.parse().expect("time did not match"),
            run_time,
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
    pub fn languages_out(&self) -> String {
        self.languages.join(", ")
    }

    pub fn repoQuery_from_repo(
        repo: &OrgViewOrganizationRepositoriesEdges,
        org: String,
        run_time: DateTime<Utc>,
    ) -> RepoQuery {
        let _repo = repo.node.as_ref().unwrap();
        return RepoQuery::new(
            _repo.name.clone(),
            org,
            _repo.created_at.parse().expect("time did not match"),
            _repo.updated_at.parse().expect("time did not match"),
            run_time,
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
