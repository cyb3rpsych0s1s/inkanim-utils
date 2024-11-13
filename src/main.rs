use clap::Parser;
use cli::CLI;

mod args;
mod cli;
mod list;
mod read;
mod show;
mod whereis;
mod whois;

use list::list;
use read::read;
use show::show;
use whereis::whereis;
use whois::whois;

fn main() {
    let args = CLI::parse();
    let files = match args {
        CLI::List(list::Args { ref files, .. }) => files,
        CLI::WhoIs(whois::Args { ref files, .. }) => files,
        CLI::WhereIs(whereis::Args { ref files, .. }) => files,
        CLI::Show(show::Args { ref files, .. }) => files,
    };
    let (widget, anim) = read(files);
    match args {
        CLI::List(args) => list(args, widget, anim),
        CLI::WhoIs(args) => whois(args, widget, anim),
        CLI::WhereIs(args) => whereis(args, widget, anim),
        CLI::Show(args) => show(args, widget, anim),
    };
}
