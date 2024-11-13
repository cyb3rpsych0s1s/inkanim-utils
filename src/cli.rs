use clap::Parser;

use crate::{list, show, whereis, whois};

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)] // requires `derive` feature
#[command(name = "inkanim", bin_name = "inkanim", author, version)]
pub enum CLI {
    /// list and optionally filter anims and target widgets
    List(list::Args),
    /// get full path names from path indexes
    #[command(name = "whois")]
    WhoIs(whois::Args),
    /// get full path indexes from path names
    #[command(name = "whereis")]
    WhereIs(whereis::Args),
    /// show a particular widget
    #[command(name = "show")]
    Show(show::Args),
}
