use anyhow::*;
use diesel::Table;
use graphql_client::*;
use log::*;
use prettytable::*;
use serde::*;
use structopt::StructOpt;

type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.graphql",
    query_path = "src/github/query_1.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "repository")]
    repo: String,
}

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

pub fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

pub fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env().context("while reading from environment")?;

    let args = Command::from_args();

    let repo = args.repo;
    let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));

    let q = RepoView::build_query(repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
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

    let response_body: Response<repo_view::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars: Option<i64> = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count);

    println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "issue", "comments"));

    for issue in &response_data
        .repository
        .expect("missing repository")
        .issues
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(issue) = issue {
            table.add_row(row!(issue.title, issue.comments.total_count));
        }
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }
}
