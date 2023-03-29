mod args;
pub(crate) use args::Args;
use inkanim::{
    anim::InkAnimAnimationLibraryResource,
    widget::{inkWidgetLibraryResource, WidgetTree},
};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

pub(crate) fn whereis(
    args: Args,
    widget: inkWidgetLibraryResource,
    anim: InkAnimAnimationLibraryResource,
) {
    let names = args
        .names
        .path
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<_>>();
    if names.is_empty() {
        panic!("please specify widget path names");
    }
    let depth = names.len();
    let sequences: Vec<&str> = anim.sequences.iter().map(|x| x.name()).collect();
    let found = widget.get_path_indexes(&names);

    if let Some(indexes) = found {
        assert_eq!(depth, names.len());

        let mut table = Table::new();
        table.style = TableStyle::rounded();
        table.add_row(Row::new(
            indexes
                .iter()
                .map(|x| TableCell::new_with_alignment(x, 1, Alignment::Center))
                .collect::<Vec<_>>(),
        ));
        table.add_row(Row::new(
            names
                .iter()
                .map(|x| TableCell::new_with_alignment(x, 1, Alignment::Center))
                .collect::<Vec<_>>(),
        ));
        println!("{}", table.render());
    } else {
        println!(
            "couldn't find\n{}\nin sequence(s): {}",
            names
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" . "),
            sequences.join(", "),
        );
    }
}
