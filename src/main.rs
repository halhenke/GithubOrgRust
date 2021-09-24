use async_std::task;
use chrono::prelude::*;
use sqlx::migrate::{MigrateError, Migrator};
use structopt::StructOpt;
use GithubOrgRust::db::info::list_orgs;
use GithubOrgRust::db::sqlx::{destroy_tables, get_connection, make_tables, upsert_org};
use GithubOrgRust::glue::update_orgs;
use GithubOrgRust::types::{Org, Repo, RepoQuery, ORGS, SQLITE_DB};

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    dotenv::dotenv()?;
    println!("Hello, world!");
    let args = Args::from_args();
    return parse_and_run(args).await;
}

#[derive(StructOpt, PartialEq, Debug)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum SubCommand {
    DestroyTables,
    ListOrgs,
    MakeTables,
    UpdateOrgs,
    RunOrg { org: String },
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
    let default_orgs = Vec::from(ORGS).iter().map(|o| o.to_string()).collect();

    println!("args are {:?}", &args);

    let e = match args.cmd {
        Some(SubCommand::RunMigrations) => migrate().await,
        Some(SubCommand::DestroyTables) => destroy_tables(&mut conn).await,
        Some(SubCommand::MakeTables) => make_tables(&mut conn).await,
        Some(SubCommand::ListOrgs) => list_orgs(),
        Some(SubCommand::UpdateOrgs) => update_orgs(&mut conn, default_orgs).await,
        Some(SubCommand::RunOrg { org }) => update_orgs(&mut conn, vec![org]).await,
        None => Err(anyhow::anyhow!(
            "This is fucked - What do you want me to do?"
        )),
    };
    return e;

}

async fn migrate() -> Result<(), anyhow::Error> {
    println!("About to run Migrations...");
    let m = Migrator::new(std::path::Path::new("./migrations")).await?;
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(SQLITE_DB)
        .await?;
    m.run(&pool).await.map_err(|e| anyhow::anyhow!(e))
}
