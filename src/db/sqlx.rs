use futures::join;
use serde::{Deserialize, Serialize};
use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::types::Json;
use sqlx::{
    ConnectOptions, Connection, Error, Executor, Pool, Sqlite, SqliteConnection, SqlitePool,
};
use std::env;
use std::str::FromStr;

use crate::types::{Org, Repo, RepoQuery, SQLITE_DB};

pub fn do_this() {
    println!("Hey this called");
}

/**
   Turns out this is probably not the right way to Create Tables as the Crate will not
   compile when they are missing - instead we use Migrations
*/
pub async fn make_tables(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {

    let mk_org = sqlx::query(
        "
    CREATE table IF NOT EXISTS org (
        name TEXT NOT NULL PRIMARY KEY,
        lastrun TEXT
    );
            ",
    );
    conn.execute(mk_org).await?;

    let mk_repo = sqlx::query(
        "
        CREATE table IF NOT EXISTS repo (
        name TEXT NOT NULL,
        org TEXT NOT NULL,
        createdAt TEXT,
        lastrun TEXT,
        PRIMARY KEY(org, name),
        FOREIGN KEY(org)
            REFERENCES org (name)
        );
            ",
    );
    conn.execute(mk_repo).await?;

    let mk_repo_query: Query<Sqlite, _> = sqlx::query!(
        "
        CREATE table IF NOT EXISTS repo_query (
            name TEXT NOT NULL,
            org TEXT NOT NULL,
            stars INTEGER,
            languages TEXT,
            topics TEXT,
            createdAt TEXT,
            updatedAt TEXT,
            lastrun TEXT,
            PRIMARY KEY(org, name, lastrun),
            FOREIGN KEY(org)
                REFERENCES org (name)
        );
            ",
    );
    conn.execute(mk_repo_query).await?;
    
    return Ok(());
}


pub async fn get_pool() -> Result<Pool<Sqlite>, anyhow::Error> {
    let db = &env::var("DATABASE_URL")?;
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(4)
        .connect(SQLITE_DB)
        .await;
    return pool.map_err(|e| anyhow::anyhow!(e));
}

pub async fn get_connection() -> Result<SqliteConnection, anyhow::Error> {
    let db = &env::var("DATABASE_URL")?;
    let mut conn = SqliteConnectOptions::from_str(db)?
        .foreign_keys(true)
        .create_if_missing(true)
        .connect()
        .await;
    return conn.map_err(|e| anyhow::anyhow!(e));
}

pub async fn destroy_tables(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    conn.execute(
        "
        DROP TABLE IF EXISTS repo_query;
        ",
    )
    .await?;
    conn.execute(
        "
        DROP TABLE IF EXISTS repo;
        ",
    )
    .await?;
    conn.execute(
        "
        DROP TABLE IF EXISTS org;
        ",
    )
    .await?;
    println!("Tables are gone Dude!!!");
    return Ok(());
}

pub async fn upsert_org(conn: &mut SqliteConnection, an_org: Org) -> Result<(), anyhow::Error> {
    let upsert = sqlx::query!(
        r#"
        INSERT INTO org(name, lastRun)
        VALUES(?, ?)
        ON CONFLICT(name)
        DO UPDATE SET lastrun=excluded.lastrun;
        "#,
        an_org.name,
        an_org.lastrun,
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
}

pub async fn upsert_repo(conn: &mut SqliteConnection, repo: Repo) -> Result<(), anyhow::Error> {
    let upsert = sqlx::query!(
        r#"
        INSERT INTO repo(name, org, createdAt, lastrun)
        VALUES(?, ?, ?, ?)
        ON CONFLICT(name, org)
        DO UPDATE SET lastrun=excluded.lastrun;
        "#,
        repo.name,
        repo.org,
        repo.created_at,
        repo.lastrun
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
    // return Ok(());
}

pub async fn upsert_repo_query(
    conn: &mut SqliteConnection,
    repo_query: RepoQuery,
) -> Result<(), anyhow::Error> {
    let languages = repo_query.languages.join(", ");
    let topics = repo_query.topics.join(", ");
    let upsert = sqlx::query!(
        r#"
        INSERT INTO repoQuery(name, org, stars, languages, topics, createdAt, updatedAt, lastrun)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(name, org, lastrun)
        DO UPDATE SET
            stars=excluded.stars,
            languages=excluded.languages,
            topics=excluded.topics,
            updatedAt=excluded.updatedAt,
            lastrun=excluded.lastrun;
        "#,
        repo_query.name,
        repo_query.org,
        repo_query.stars,
        languages,
        topics,
        repo_query.created_at,
        repo_query.updated_at,
        repo_query.lastrun,
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
}

pub async fn upsert_tuple(
    conn: &mut SqliteConnection,
    org: Org,
    repos: Vec<Repo>,
    repo_queries: Vec<RepoQuery>,
) -> Result<(), anyhow::Error> {
    // join!(upsert_org(conn, org));
    upsert_org(conn, org).await?;
    for repo in repos {
        upsert_repo(conn, repo).await?;
    }
    for repo_query in repo_queries {
        upsert_repo_query(conn, repo_query).await?;
    }
    return Ok(());
}
