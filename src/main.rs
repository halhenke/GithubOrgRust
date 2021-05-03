// pub mod lib;
// #[feature(type_name_of_val)]
// extern crate GithubOrgRust;
// extern crate async;
use async_std::task;
use chrono::prelude::*;
use structopt::StructOpt;
use GithubOrgRust::db::sqlx::{destroy_tables, get_connection, make_tables, upsert_org};
use GithubOrgRust::types::{Org, Repo, RepoQuery, SQLITE_DB};

use sqlx::migrate::{MigrateError, Migrator};

// static MIGRATOR: Migrator = sqlx::migrate!();

#[async_std::main]
// async fn main(args: Args) -> Result<(), anyhow::Error> {
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    println!("Hello, world!");
    let args = Args::from_args();
    // migrate().await?;
    // GithubOrgRust::db::sqlx::do_this();
    // GithubOrgRust::github::example::main()
    // task::block_on(GithubOrgRust::db::sqlx::connect_db());
    return parse_and_run(args).await;
    // return Ok(());
}

// #[derive(StructOpt)]
// #[structopt(author, about)]
// struct Command {
//     #[structopt(name = "organization")]
//     org: String,
// }

#[derive(StructOpt, PartialEq, Debug)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum SubCommand {
    DestroyTables,
    MakeTables,
    UpdateOrgs,
    RunMigrations,
}

/**
    Parse the Command Line Arguments using StructOpt and run the appropriate
    Command/Function.
*/
async fn parse_and_run(args: Args) -> Result<(), anyhow::Error> {
    let mut conn = get_connection().await?;
    let org_defaults = "Google".to_string();
    let org = Org::new(org_defaults, Utc::now());
    let e = match args.cmd {
        Some(DestroyTables) => destroy_tables(&mut conn).await,
        Some(MakeTables) => make_tables(&mut conn).await,
        Some(UpdateOrgs) => upsert_org(&mut conn, org).await,
        Some(RunMigrations) => migrate().await,
        // None => ()
        None => Err(anyhow::anyhow!(
            "This is fucked - What do you want me to do?"
        )),
    };
    return e;
    // return Ok(e);
    // return Ok::Result<(), anyhow::Error>(e);
    // return GithubOrgRust::github::orgQuery::github_query_from_main("Google".to_string());
}

async fn migrate() -> Result<(), anyhow::Error> {
    println!("About to run Migrations...");
    let m = Migrator::new(std::path::Path::new("./migrations")).await?;
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(SQLITE_DB)
        .await?;
    m.run(&pool).await.map_err(|e| anyhow::anyhow!(e))
}

// /// Supposed to work if we roughly pass the Github Org from the Command Line
// pub fn query_org_from_cli() -> Result<(), anyhow::Error> {
//     // dotenv::dotenv().ok();
//     // env_logger::init();

//     // let config: Env = envy::from_env().context("while reading from environment")?;
//     let args = Command::from_args();

//     let org = args.org;
//     // let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));
//     return query_org(org);
// }
