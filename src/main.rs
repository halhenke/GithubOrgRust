// pub mod lib;
// #[feature(type_name_of_val)]
// extern crate GithubOrgRust;
// extern crate async;
use async_std::task;

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    println!("Hello, world!");
    // GithubOrgRust::db::sqlx::do_this();
    // GithubOrgRust::github::example::main()
    task::block_on(GithubOrgRust::db::sqlx::connect_db());
    return GithubOrgRust::github::orgQuery::github_query_from_main("Google".to_string());
}
