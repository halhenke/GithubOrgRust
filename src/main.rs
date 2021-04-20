// pub mod lib;
// extern crate GithubOrgRust;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    // GithubOrgRust::db::sqlx::do_this();
    GithubOrgRust::github::example::main()
}
