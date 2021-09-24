use super::github::org_query::query_org;
use crate::db::sqlx::{get_connection, upsert_tuple};
use crate::types::{Org, Repo, RepoQuery};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};

type QueryResult = (Org, Vec<Repo>, Vec<RepoQuery>);

pub async fn update_orgs(
    conn: &mut SqliteConnection,
    orgs: Vec<String>,
) -> Result<(), anyhow::Error> {
    for org_name in &orgs {
        println!("Org is {}", org_name);
        let (org, repos, repo_queries) = query_org(org_name.to_string()).await?;
        upsert_tuple(conn, org, repos, repo_queries).await?;
    }
    return Ok(());
}
