use super::github::orgQuery::query_org;
use crate::db::sqlx::{get_connection, upsert_tuple};
use crate::types::{Org, Repo, RepoQuery};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};

type QueryResult = (Org, Vec<Repo>, Vec<RepoQuery>);

pub async fn update_orgs(
    conn: &mut SqliteConnection,
    orgs: Vec<String>,
) -> Result<(), anyhow::Error> {
    // let results (repos, repoQueries) =
    // let results: Vec<QueryResult> = orgs
    //     .into_iter()
    //     .map(async move |org| query_org(org).await.iter())
    //     // .await
    //     .collect();
    for orgName in &orgs {
        println!("Org is {}", orgName);
        let (org, repos, repoQueries) = query_org(orgName.to_string()).await?;
        upsert_tuple(conn, org, repos, repoQueries).await?;
    }
    return Ok(());
}
