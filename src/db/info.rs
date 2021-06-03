use crate::types::ORGS;
use anyhow::{Context, Error};
use prettytable::{cell, row};

pub fn list_orgs() -> Result<(), Error> {
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!("Orgs"));
    for org in ORGS.iter() {
        table.add_row(row!(Fg-> org));
    }
    table.printstd();
    return Ok(());
}
