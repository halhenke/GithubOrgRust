// pub mod lib;
// extern crate GithubOrgRust;
// extern crate async;
use async_std::task;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    // GithubOrgRust::db::sqlx::do_this();
    // GithubOrgRust::github::example::main()
    task::block_on(GithubOrgRust::db::sqlx::connect_db())
}
