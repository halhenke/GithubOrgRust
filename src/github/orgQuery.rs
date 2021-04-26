use anyhow::{Context, Error};
use graphql_client::{GraphQLQuery, Response};
use prettytable::{cell, row};
// use super::example::parse_repo_name;
use log::{info, warn};
use serde::{Deserialize, Deserializer};
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
struct OrgView;

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

pub fn query_org(org: String) -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

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

    // let stars: Option<i64> = response_data
    //     .repository
    //     .as_ref()
    //     .map(|repo| repo.stargazers.total_count);

    // println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "Org", "Repo", "Stars"));

    for repo in repositories.edges.expect("Repository Nodes is NULL")
    // .nodes
    // .expect()
    {
        if let Some(repo) = repo {
            println!("Type of repo is {}", std::any::type_name_of_val(&repo));
            table.add_row(row!(&org, repo.node.unwrap().name, "Hooooo"));
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

// use GithubOrgRust::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges;

// pub fn get_stars(
//     repo: root::github::orgQuery::org_view::OrgViewOrganizationRepositoriesEdges,
// ) -> Option<i64> {
//     let stars: Option<i64> = repo.as_ref().map(|repo| repo.stargazers.total_count);
//     return stars;
// }
// pub fn get_stars<T>(repo: T) -> Option<i64> {
//     let stars: Option<i64> = repo.as_ref().map(|repo| repo.stargazers.total_count);
//     return stars;
// }
