use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{
    ConnectOptions, Connection, Error, Executor, Pool, Sqlite, SqliteConnection, SqlitePool,
};
use std::env;
use std::str::FromStr;

pub fn do_this() {
    println!("Hey this called");
}

const SQLITE_DB: &'static str = "sqlite://rust-git-org.sqlite";

// const WORDS: &'static str = "hello rust!";

pub async fn connect_db() -> Result<(), anyhow::Error> {
    // pub async fn connect_db() -> Result<SqlitePool, anyhow::Error> {
    // let con = Connection::connect(url)
    // let conn = SQLiteConnection::connect()
    // let p = SqlitePool::
    let db = &env::var("DATABASE_URL")?;
    let mut conn = SqliteConnectOptions::from_str(db)?
        // let mut conn = SqliteConnectOptions::new()
        //     .filename(SQLITE_DB)
        .foreign_keys(true)
        .create_if_missing(true)
        .connect()
        .await?;
    // let pool = SqlitePool::connect(SQLITE_DB).await?;
    // pool.
    // conn.execute("PRAGMA foreign_keys = ON;").await?;
    // pool.execute("PRAGMA foreign_keys = ON;").await?;

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
        created TEXT,
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
            created TEXT,
            updated TEXT,
            lastrun TEXT,
            PRIMARY KEY(org, name),
            FOREIGN KEY(org)
                REFERENCES org (name)
        );
            ",
    );
    conn.execute(mkRepoQuery).await?;
    // pool.execute(mkRepoQuery).await?;

    // pool.execute(sqlx::query("SELECT"));

    // return Ok(pool);
    return Ok(());
}
