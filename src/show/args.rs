use crate::args::{Files, OptionalPathIndexes, OptionalPathNames};

#[derive(clap::Args, Debug)]
#[command()]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub indexes: OptionalPathIndexes,

    #[command(flatten)]
    pub names: OptionalPathNames,
}
