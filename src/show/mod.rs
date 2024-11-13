mod args;
pub(crate) use args::Args;
use inkanim::{
    anim::InkAnimAnimationLibraryResource,
    widget::{inkWidgetLibraryResource, ByName},
    RedsWidget,
};

fn reds(widget: &inkWidgetLibraryResource, names: &[&str]) {
    if names.is_empty() {
        panic!("please specify the names tree");
    }
    let (first, others) = names.split_at(1);
    let (_, mut parent) = widget
        .root_chunk()
        .root_widget
        .by_name(first.first().unwrap())
        .unwrap_or_else(|| {
            panic!("unable to find widget in tree: {names:#?}");
        });
    let last_idx = names.len() - 1;
    for (idx, name) in others.iter().enumerate() {
        if parent.is_leaf() && idx < last_idx {
            panic!("unable to find widget in tree: {names:#?}");
        }
        (_, parent) = parent
            .as_compound()
            .unwrap()
            .by_name(name)
            .unwrap_or_else(|| {
                panic!("unable to find widget in tree: {names:#?}");
            });
    }
    println!("{}", parent.reds_widget("root", None));
}

pub(crate) fn show(
    args: Args,
    widget: inkWidgetLibraryResource,
    _anim: InkAnimAnimationLibraryResource,
) {
    match (args.indexes.path, args.names.names) {
        (None, None) | (Some(_), Some(_)) => panic!("please specify either --indexes or --names"),
        (None, Some(names)) => reds(
            &widget,
            names
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        (Some(_indexes), None) => todo!(),
    }
}
