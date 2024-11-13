use std::path::PathBuf;

use clap::ValueEnum;
use inkanim::{
    anim::{InkAnimAnimationLibraryResource, InkAnimInterpolatorType},
    widget::inkWidgetLibraryResource,
};

pub struct DualResources {
    pub widget: inkWidgetLibraryResource,
    pub anim: InkAnimAnimationLibraryResource,
    pub filter_by_path: Option<Vec<usize>>,
    pub filter_by_type: Option<InkAnimInterpolatorType>,
    pub show_path_names: bool,
}

#[derive(clap::Args, Debug)]
pub struct Files {
    /// .inkwidget path
    #[arg(short, long, value_name = "FILE")]
    pub widget: PathBuf,

    /// optional .inkanim path
    ///
    /// note: if left unspecified, it defaults to the same path as the .inkwidget,
    /// with suffix "_animations" and .inkanim extension instead
    #[arg(short, long, value_name = "FILE")]
    pub anim: Option<PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct PathIndexes {
    /// filter by widget path indexes
    ///
    /// e.g. "1.3.0.0.16"
    #[arg(short, long, value_parser = parse_path_indexes, value_name = "INDEXES")]
    pub path: std::vec::Vec<usize>,
}

#[derive(clap::Args, Debug)]
pub struct OptionalPathIndexes {
    /// filter by widget path indexes
    ///
    /// e.g. "1.3.0.0.16"
    #[arg(short, long, value_parser = parse_path_indexes, value_name = "INDEXES")]
    pub path: Option<std::vec::Vec<usize>>,
}

#[derive(clap::Args, Debug)]
pub struct PathNames {
    /// filter by widget path name(s)
    ///
    /// e.g. "main_canvas.Arrival.Arrival_GPS_Canvas.Arrival_GPS_Elements_Canvas"
    #[arg(short, long, value_parser = parse_path_names, value_name = "NAMES")]
    pub path: std::vec::Vec<String>,
}

#[derive(clap::Args, Debug)]
pub struct OptionalPathNames {
    /// filter by widget path name(s)
    ///
    /// e.g. "main_canvas.Arrival.Arrival_GPS_Canvas.Arrival_GPS_Elements_Canvas"
    #[arg(short, long, value_parser = parse_path_names, value_name = "NAMES")]
    pub names: Option<std::vec::Vec<String>>,
}

#[derive(clap::Args, Debug)]
pub struct Mode {
    /// optionally output as JSON, Redscript or table (default)
    #[arg(value_enum, long, default_value_t = Output::Table)]
    pub output: Output,
}

pub(crate) fn parse_path_indexes(path: &str) -> Result<Vec<usize>, std::io::Error> {
    Ok(path
        .split(&['.', '│'][..])
        .map(|x| x.trim().parse::<usize>().expect("digit"))
        .collect())
}

pub(crate) fn parse_path_names(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(path
        .split(&['.', '│'][..])
        .map(|x| x.trim().to_string())
        .collect())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Output {
    Table,
    Json,
    Reds,
}
