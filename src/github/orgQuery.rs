use anyhow::{Context, Error};
use graphql_client::{GraphQLQuery, Response};
use prettytable::{cell, row};
// use super::example::parse_repo_name;
use crate::types::{Org, Repo, RepoQuery};
use chrono::prelude::*;
use log::{info, warn};
use serde::{Deserialize, Deserializer};
use std::iter::Iterator;
use structopt::StructOpt;

type URI = String;
type HTML = String;
type DateTime = String;
// type DateTime = std::time::SystemTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.graphql",
    query_path = "src/github/orgQuery.graphql",
    response_derives = "Debug"
)]
pub struct OrgView;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "organization")]
    org: String,
}

#[derive(Debug, Deserialize)]
struct Env {
    github_api_token: String,
}

/// Supposed to work if we roughly pass the Github Org from the Command Line
pub fn query_org_from_cli() -> Result<(), anyhow::Error> {
    // dotenv::dotenv().ok();
    // env_logger::init();

    // let config: Env = envy::from_env().context("while reading from environment")?;
    let args = Command::from_args();

    let org = args.org;
    // let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));
    return query_org(org);
}

pub fn github_query_from_main(org: String) -> Result<(), anyhow::Error> {
    return query_org(org);
}

/// Run the Github Org Query and Process the Results
pub fn query_org(org: String) -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut repos: Vec<Repo> = Vec::new();
    let mut repoQueries: Vec<RepoQuery> = Vec::new();

    let config: Env = envy::from_env().context("while reading from environment")?;

    let q = OrgView::build_query(org_view::Variables { org: org.clone() });

    let client = reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build()?;

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&q)
        .send()?;

    res.error_for_status_ref()?;

    let response_body: Response<org_view::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    let response_data: org_view::ResponseData = response_body.data.expect("missing response data");
    let repositories = response_data
        .organization
        .expect("missing organization")
        .repositories;

    // println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "Org", "Repo", "Stars"));

    for repo in repositories.edges.expect("Repository Nodes is NULL")
    // .nodes
    // .expect()
    {
        if let Some(repo) = repo {
            println!("Repo: {:?}", Repo::repo_from_repo(&repo, org.clone()));
            let repoStruct: Repo = Repo::repo_from_repo(&repo, org.clone());
            let repoQueryStruct: RepoQuery = RepoQuery::repoQuery_from_repo(&repo, org.clone());
            repos.push(repoStruct);
            repoQueries.push(repoQueryStruct);
            let r = repo.node.expect("missing");
            let stars: i64 = r.stargazers.total_count;
            // .expect("Stars");
            // .ok_or(anyhow::Error)
            // .as_ref()
            // .map(|repo| repo.stargazers.total_count)
            // .ok_or(anyhow::Error())?;

            // println!("Type of repo is {}", std::any::type_name_of_val(&repo));
            table.add_row(row!(&org, r.name, stars));
        }
    }
    table.printstd();

    // for issue in &response_data
    //     .repository
    //     .expect("missing repository")
    //     .issues
    //     .nodes
    //     .expect("issue nodes is null")
    // {
    //     if let Some(issue) = issue {
    //         table.add_row(row!(issue.title, issue.comments.total_count));
    //     }
    // }

    // table.printstd();
    Ok(())
}

// pub fn repo_to_repo(
//     repo: &super::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges,
// ) -> Repo {
//     return Repo {
//         name: "Repo".to_string(),
//         org: "Google".to_string(),
//         createdAt: std::time::SystemTime::now(),
//         lastrun: std::time::SystemTime::now(),
//     };
// }

// use GithubOrgRust::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges;

// pub fn get_stars(
//     repo: root::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges,
// ) -> Option<i64> {
//     let stars: Option<i64>: repo.as_ref().map(|repo| repo.stargazers.total_count);
//     return stars;
// }
// pub fn get_stars<T>(repo: T) -> Option<i64> {
//     let stars: Option<i64> = repo.as_ref().map(|repo| repo.stargazers.total_count);
//     return stars;
// }
