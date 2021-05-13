// use futures_core::future::BoxFuture;
// use futures::future::BoxFuture;
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

// const WORDS: &'static str = "hello rust!";

/**
   Turns out this is probably not the right way to Create Tables as the Crate will not
   compile when they are missing - instead we use Migrations
*/
pub async fn make_tables(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    // // pub async fn connect_db() -> Result<SqlitePool, anyhow::Error> {
    // // let con = Connection::connect(url)
    // // let conn = SQLiteConnection::connect()
    // // let p = SqlitePool::
    // let db = &env::var("DATABASE_URL")?;
    // let mut conn = SqliteConnectOptions::from_str(db)?
    //     // let mut conn = SqliteConnectOptions::new()
    //     //     .filename(SQLITE_DB)
    //     .foreign_keys(true)
    //     .create_if_missing(true)
    //     .connect()
    //     .await?;
    // // let pool = SqlitePool::connect(SQLITE_DB).await?;
    // // pool.
    // // conn.execute("PRAGMA foreign_keys = ON;").await?;
    // // pool.execute("PRAGMA foreign_keys = ON;").await?;
    // let mut conn = get_connection().await?;

    let mkOrg = sqlx::query(
        "
    CREATE table IF NOT EXISTS org (
        name TEXT NOT NULL PRIMARY KEY,
        lastrun TEXT
    );
            ",
    );
    conn.execute(mkOrg).await?;
    // pool.execute(mkOrg).await?;

    let mkRepo = sqlx::query(
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
    conn.execute(mkRepo).await?;
    // pool.execute(mkRepo).await?;

    let mkRepoQuery: Query<Sqlite, _> = sqlx::query!(
        "
        CREATE table IF NOT EXISTS repoQuery (
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
    conn.execute(mkRepoQuery).await?;
    // pool.execute(mkRepoQuery).await?;

    // pool.execute(sqlx::query("SELECT"));

    println!("Tables are back dude!");
    // return Ok(pool);
    return Ok(());
}

// struct Row {
//     id: i64,
//     person: Json<Org>,
// }

pub async fn get_pool() -> Result<Pool<Sqlite>, anyhow::Error> {
    let db = &env::var("DATABASE_URL")?;
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(4)
        .connect(SQLITE_DB)
        .await;
    return pool.map_err(|e| anyhow::anyhow!(e));
}

pub async fn get_connection() -> Result<SqliteConnection, anyhow::Error> {
    // pub async fn get_connection() -> Future<Result<SqliteConnection, anyhow::Error>> {
    let db = &env::var("DATABASE_URL")?;
    let mut conn = SqliteConnectOptions::from_str(db)?
        // let mut conn = SqliteConnectOptions::new()
        //     .filename(SQLITE_DB)
        .foreign_keys(true)
        .create_if_missing(true)
        .connect()
        .await;
    return conn.map_err(|e| anyhow::anyhow!(e));
}

pub async fn destroy_tables(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    // println!("DestroyTables Called");
    conn.execute(
        "
        DROP TABLE IF EXISTS repoQuery;
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

pub async fn upsert_org(conn: &mut SqliteConnection, anOrg: Org) -> Result<(), anyhow::Error> {
    let upsert = sqlx::query!(
        r#"
        INSERT INTO org(name, lastRun)
        VALUES(?, ?)
        ON CONFLICT(name)
        DO UPDATE SET lastrun=excluded.lastrun;
        "#,
        anOrg.name,
        anOrg.lastrun,
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
    // return Ok(());
    // return upsert.fetch_one(conn).await?;
    // .bind(org);
    // return conn.execute(upsert).await?;
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
        repo.createdAt,
        repo.lastrun
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
    // return Ok(());
}

pub async fn upsert_repoQuery(
    conn: &mut SqliteConnection,
    repoQuery: RepoQuery,
) -> Result<(), anyhow::Error> {
    let languages = repoQuery.languages.join(", ");
    let topics = repoQuery.topics.join(", ");
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
        repoQuery.name,
        repoQuery.org,
        repoQuery.stars,
        languages,
        topics,
        repoQuery.createdAt,
        repoQuery.updatedAt,
        repoQuery.lastrun,
    )
    .fetch_optional(conn)
    .await;
    return upsert.and(Ok(())).map_err(|e| anyhow::anyhow!(e));
}

pub async fn upsert_tuple(
    conn: &mut SqliteConnection,
    org: Org,
    repos: Vec<Repo>,
    repoQueries: Vec<RepoQuery>,
) -> Result<(), anyhow::Error> {
    // join!(upsert_org(conn, org));
    upsert_org(conn, org).await?;
    // repos.into_iter().map(|r| upsert_repo(conn, r));
    for repo in repos {
        upsert_repo(conn, repo).await?;
    }
    for repoQuery in repoQueries {
        upsert_repoQuery(conn, repoQuery).await?;
    }
    return Ok(());
}

// pub async fn upsert_org(conn: &SqliteConnection, anOrg: Org) -> Result<(), anyhow::Error> {
//     let upsert = sqlx::query!(
//         r#"
//         INSERT INTO org(anOrg)
//         VALUES($1)
//         ON CONFLICT(name)
//         DO UPDATE SET lastrun=excluded.lastrun;
//         "#,
//         Json(anOrg) as _
//     )
//     .fetch_one(conn)
//     .await?;
//     return Ok(());
//     // return upsert.fetch_one(conn).await?;
//     // .bind(org);
//     // return conn.execute(upsert).await?;
// }
