use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Connection, Error, Executor, Pool, Sqlite, SqlitePool};



pub fn do_this() {
    println!("Hey this called");
}

pub async fn connect_db() -> Result<SqlitePool, Error> {
    // let con = Connection::connect(url)
    // let conn = SQLiteConnection::connect()
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    // pool.
    pool.execute("PRAGMA foreign_keys = ON;").await?;

    let mkOrg = sqlx::query(
        "
    CREATE table org (
        name TEXT NOT NULL PRIMARY KEY,
        lastrun TEXT
    ",
    );
    pool.execute(mkOrg).await?;

    let mkRepo: Query<Sqlite, _> = sqlx::query(
        "
        CREATE table repo (
            name TEXT NOT NULL,
            org TEXT NOT NULL,
            # stars INTEGER,
            # language TEXT,
            created TEXT,
            # updated TEXT,
            lastrun TEXT,
            PRIMARY KEY(org, name),
            FOREIGN KEY(org)
                REFERENCES org (name)
        )
    ",
    );

    // pool.execute(sqlx::query("SELECT"));

    return Ok(pool);
}
