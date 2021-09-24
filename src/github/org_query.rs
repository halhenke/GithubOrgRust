use anyhow::{Context, Error};
use graphql_client::{GraphQLQuery, Response};
use prettytable::{cell, row};
use crate::types::{Org, Repo, RepoQuery};
use chrono::prelude::*;
use log::{info, warn};
use serde::{Deserialize, Deserializer};
use std::iter::Iterator;
use structopt::StructOpt;

type URI = String;
type HTML = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.graphql",
    query_path = "src/github/orgQuery.graphql",
    response_derives = "Debug"
)]
pub struct OrgView;

#[derive(Debug, Deserialize)]
struct Env {
    github_api_token: String,
}

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "organization")]
    org: String,
}


/// Run the Github Org Query and Process the Results
pub async fn query_org(org_name: String) -> Result<(Org, Vec<Repo>, Vec<RepoQuery>), anyhow::Error> {
    dotenv::dotenv().ok();

    let mut repos: Vec<Repo> = Vec::new();
    let mut repo_queries: Vec<RepoQuery> = Vec::new();

    let config: Env = envy::from_env().context("while reading from environment")?;

    let q = OrgView::build_query(org_view::Variables {
        org: org_name.clone(),
    });

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

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "Org", "Repo", "Stars", "Updated At"));
    let time_now = Utc::now();

    let org = Org::new(org_name.clone(), time_now);

    for repo in repositories.edges.expect("Repository Nodes is NULL")
    {
        if let Some(repo) = repo {
            let repo_struct: Repo = Repo::repo_from_repo(&repo, org_name.clone(), time_now);
            let repo_query_struct: RepoQuery =
                RepoQuery::repo_query_from_repo(&repo, org_name.clone(), time_now);
            repos.push(repo_struct);
            repo_queries.push(repo_query_struct);
            let r = repo.node.expect("missing");
            let stars: i64 = r.stargazers.total_count;
            let u_at: chrono::DateTime<Utc> = r.updated_at.parse().expect("kialler");

            table.add_row(row!(Fg-> &org_name, Fg->r.name, Fy->stars, Fg->u_at));
        }
    }
    table.printstd();

    return Ok((org, repos, repo_queries));
}