use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

use crate::{ink::InkWrapper, CName};

use super::{
    inkBorderWidget, inkCacheWidget, inkCanvasWidget, inkCircleWidget, inkFlexWidget,
    inkHorizontalPanelWidget, inkImageWidget, inkMaskWidget, inkMultiChildren, inkRectangleWidget,
    inkScrollAreaWidget, inkShapeWidget, inkTextWidget, inkUniformGridWidget,
    inkVectorGraphicWidget, inkVerticalPanelWidget, inkVideoWidget, inkVirtualCompoundWidget,
    inkWidgetLibraryItem, inkWidgetLibraryItemInstance, inkWidgetLibraryResource, SiblingOrNested,
    Widget, WidgetSummary,
};

impl SiblingOrNested for Vec<usize> {
    fn sibling_or_nested(&self, searched: &[usize]) -> bool {
        let count_own = self.len();
        let count_searched = searched.len();
        if count_searched == 0 {
            return true;
        }
        let last_searched = count_searched - 1;
        for (i, path_index) in self.iter().enumerate() {
            if *path_index != searched[i] {
                return false;
            }
            if i == last_searched && count_own >= count_searched {
                return true;
            }
        }
        false
    }
}

macro_rules! impl_ink_children {
    ($ty:ident) => {
        impl InkChildren for $ty {
            fn orphans(&self) -> Vec<Widget> {
                self.children.data.orphans()
            }

            fn children(&self) -> Vec<InkWrapper<Widget>> {
                self.children.data.children()
            }
        }
    };
}

macro_rules! impl_ink_widget {
    ($ty:ident) => {
        impl InkWidget for $ty {
            fn name(&self) -> &str {
                self.name.as_str()
            }
        }
    };
}

macro_rules! impl_leaf_widget {
    ($ty:ident) => {
        impl InkLeafWidget for $ty {}
    };
}

macro_rules! impl_classname {
    ($ty:ident) => {
        impl Classname for $ty {
            fn classname(&self) -> String {
                stringify!($ty).to_string()
            }
        }
    };
}

#[enum_dispatch]
pub trait Classname {
    fn classname(&self) -> String;
}

pub trait InkWidget: Debug {
    fn name(&self) -> &str;
}

pub trait InkChildren {
    fn orphans(&self) -> Vec<Widget>;
    fn children(&self) -> Vec<InkWrapper<Widget>>;
}

pub trait InkLeafWidget: InkWidget + Debug {}

pub trait InkCompoundWidget: InkWidget + InkChildren + Debug {}

impl<T> InkCompoundWidget for T where T: InkWidget + InkChildren + Debug {}

impl InkChildren for inkMultiChildren {
    fn orphans(&self) -> Vec<Widget> {
        self.children.iter().map(|x| x.data.clone()).collect()
    }

    fn children(&self) -> Vec<InkWrapper<Widget>> {
        self.children.to_vec()
    }
}

impl_ink_children!(inkCanvasWidget);
impl_ink_children!(inkHorizontalPanelWidget);
impl_ink_children!(inkVerticalPanelWidget);
impl_ink_children!(inkScrollAreaWidget);
impl_ink_children!(inkUniformGridWidget);
impl_ink_children!(inkVirtualCompoundWidget);
impl_ink_children!(inkFlexWidget);
impl_ink_children!(inkCacheWidget);

impl_ink_widget!(inkCanvasWidget);
impl_ink_widget!(inkHorizontalPanelWidget);
impl_ink_widget!(inkVerticalPanelWidget);
impl_ink_widget!(inkScrollAreaWidget);
impl_ink_widget!(inkUniformGridWidget);
impl_ink_widget!(inkVirtualCompoundWidget);
impl_ink_widget!(inkFlexWidget);
impl_ink_widget!(inkCacheWidget);

impl_ink_widget!(inkTextWidget);
impl_ink_widget!(inkImageWidget);
impl_ink_widget!(inkVideoWidget);
impl_ink_widget!(inkMaskWidget);
impl_ink_widget!(inkBorderWidget);
impl_ink_widget!(inkShapeWidget);
impl_ink_widget!(inkCircleWidget);
impl_ink_widget!(inkRectangleWidget);
impl_ink_widget!(inkVectorGraphicWidget);

impl_leaf_widget!(inkTextWidget);
impl_leaf_widget!(inkImageWidget);
impl_leaf_widget!(inkVideoWidget);
impl_leaf_widget!(inkMaskWidget);
impl_leaf_widget!(inkBorderWidget);
impl_leaf_widget!(inkShapeWidget);
impl_leaf_widget!(inkCircleWidget);
impl_leaf_widget!(inkRectangleWidget);
impl_leaf_widget!(inkVectorGraphicWidget);

impl_classname!(inkMultiChildren);

impl_classname!(inkCanvasWidget);
impl_classname!(inkHorizontalPanelWidget);
impl_classname!(inkVerticalPanelWidget);
impl_classname!(inkScrollAreaWidget);
impl_classname!(inkUniformGridWidget);
impl_classname!(inkVirtualCompoundWidget);
impl_classname!(inkFlexWidget);
impl_classname!(inkCacheWidget);

impl_classname!(inkTextWidget);
impl_classname!(inkImageWidget);
impl_classname!(inkVideoWidget);
impl_classname!(inkMaskWidget);
impl_classname!(inkBorderWidget);
impl_classname!(inkShapeWidget);
impl_classname!(inkCircleWidget);
impl_classname!(inkRectangleWidget);
impl_classname!(inkVectorGraphicWidget);

impl Widget {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::inkMultiChildren(_) => None,
            Self::inkCanvasWidget(node) => Some(node.name()),
            Self::inkHorizontalPanelWidget(node) => Some(node.name()),
            Self::inkVerticalPanelWidget(node) => Some(node.name()),
            Self::inkScrollAreaWidget(node) => Some(node.name()),
            Self::inkUniformGridWidget(node) => Some(node.name()),
            Self::inkVirtualCompoundWidget(node) => Some(node.name()),
            Self::inkFlexWidget(node) => Some(node.name()),
            Self::inkCacheWidget(node) => Some(node.name()),
            Self::inkTextWidget(node) => Some(node.name()),
            Self::inkImageWidget(node) => Some(node.name()),
            Self::inkVideoWidget(node) => Some(node.name()),
            Self::inkMaskWidget(node) => Some(node.name()),
            Self::inkBorderWidget(node) => Some(node.name()),
            Self::inkShapeWidget(node) => Some(node.name()),
            Self::inkCircleWidget(node) => Some(node.name()),
            Self::inkRectangleWidget(node) => Some(node.name()),
            Self::inkVectorGraphicWidget(node) => Some(node.name()),
        }
    }
    pub fn as_compound(&self) -> Option<&dyn InkCompoundWidget> {
        match self {
            Self::inkCanvasWidget(node) => Some(node),
            Self::inkHorizontalPanelWidget(node) => Some(node),
            Self::inkVerticalPanelWidget(node) => Some(node),
            Self::inkScrollAreaWidget(node) => Some(node),
            Self::inkUniformGridWidget(node) => Some(node),
            Self::inkVirtualCompoundWidget(node) => Some(node),
            Self::inkFlexWidget(node) => Some(node),
            Self::inkCacheWidget(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_widget(&self) -> Option<&dyn InkWidget> {
        match self {
            Self::inkMultiChildren(_) => None,
            Self::inkCanvasWidget(widget) => Some(widget),
            Self::inkHorizontalPanelWidget(widget) => Some(widget),
            Self::inkVerticalPanelWidget(widget) => Some(widget),
            Self::inkScrollAreaWidget(widget) => Some(widget),
            Self::inkUniformGridWidget(widget) => Some(widget),
            Self::inkVirtualCompoundWidget(widget) => Some(widget),
            Self::inkFlexWidget(widget) => Some(widget),
            Self::inkCacheWidget(widget) => Some(widget),
            Self::inkTextWidget(widget) => Some(widget),
            Self::inkImageWidget(widget) => Some(widget),
            Self::inkVideoWidget(widget) => Some(widget),
            Self::inkMaskWidget(widget) => Some(widget),
            Self::inkBorderWidget(widget) => Some(widget),
            Self::inkShapeWidget(widget) => Some(widget),
            Self::inkCircleWidget(widget) => Some(widget),
            Self::inkRectangleWidget(widget) => Some(widget),
            Self::inkVectorGraphicWidget(widget) => Some(widget),
        }
    }
    pub fn as_leaf(&self) -> Option<&dyn InkLeafWidget> {
        match self {
            Self::inkTextWidget(widget) => Some(widget),
            Self::inkImageWidget(widget) => Some(widget),
            Self::inkVideoWidget(widget) => Some(widget),
            Self::inkMaskWidget(widget) => Some(widget),
            Self::inkBorderWidget(widget) => Some(widget),
            Self::inkShapeWidget(widget) => Some(widget),
            Self::inkCircleWidget(widget) => Some(widget),
            Self::inkRectangleWidget(widget) => Some(widget),
            Self::inkVectorGraphicWidget(widget) => Some(widget),
            _ => None,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.as_leaf().is_some()
    }
    pub fn is_compound(&self) -> bool {
        self.as_compound().is_some()
    }
}

pub trait WidgetTree {
    /// return the widget type
    fn get_widget_classname(&self, path: &[usize]) -> Option<String>;
    /// return the full path names to the widget
    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>>;
    /// return the full path indexes to the widget
    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>>;
}

pub trait ByIndex {
    /// find a widget by index
    fn by_index(&self, idx: usize) -> Option<Widget>;
}

pub trait ByName {
    /// find a widget by name
    fn by_name(&self, name: &str) -> Option<(usize, Widget)>;
}

pub trait Leaves {
    /// get widget summary for elements
    fn leaves(&self) -> Vec<WidgetSummary>;
}

impl<T> InkChildren for InkWrapper<T>
where
    T: InkChildren,
{
    fn orphans(&self) -> Vec<Widget> {
        self.data.orphans()
    }

    fn children(&self) -> Vec<InkWrapper<Widget>> {
        self.data.children()
    }
}

impl<T> ByIndex for T
where
    T: InkChildren,
{
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.orphans().get(idx).cloned()
    }
}

impl<T> ByName for T
where
    T: InkChildren,
{
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        for (idx, child) in self.orphans().iter().enumerate() {
            if let Widget::inkMultiChildren(_) = &child {
                panic!("unexpected inkMultiChildren with name {name}");
            }
            if let Some(compound) = child.as_compound() {
                if compound.name() == name {
                    return Some((idx, child.clone()));
                }
            }
            continue;
        }
        None
    }
}

impl<T> Leaves for T
where
    T: InkCompoundWidget,
{
    fn leaves(&self) -> Vec<WidgetSummary> {
        let mut out = vec![];
        for child in self.children().iter() {
            if let Some(name) = child.data.name() {
                out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: CName(name.to_string()),
                });
            }
        }
        out
    }
}

impl ByName for Vec<InkWrapper<Widget>> {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        for (idx, widget) in self.iter().enumerate() {
            if let Some(compound) = widget.data.as_compound() {
                if compound.name() == name {
                    return Some((idx, widget.data.clone()));
                }
            }
            if let Some(leaf) = widget.data.as_leaf() {
                if leaf.name() == name {
                    return Some((idx, widget.data.clone()));
                }
            }
        }
        None
    }
}

impl ByIndex for Vec<InkWrapper<Widget>> {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.get(idx).map(|x| x.data.clone())
    }
}

impl ByName for &dyn InkCompoundWidget {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        self.children().by_name(name)
    }
}

impl ByIndex for &dyn InkCompoundWidget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children().by_index(idx)
    }
}

impl ByIndex for Widget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        if let Widget::inkMultiChildren(node) = self {
            return node.by_index(idx);
        }
        if let Some(compound) = self.as_compound() {
            return compound.children().by_index(idx);
        }
        Some(self.clone())
    }
}

impl WidgetTree for inkWidgetLibraryItemInstance {
    fn get_widget_classname(&self, path: &[usize]) -> Option<String> {
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));
        let last = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.as_ref().unwrap().by_index(*idx) {
                if let Widget::inkMultiChildren(_) = child {
                    panic!("encountered unexpected inkMultiChildren at index {idx}");
                }
                if child.as_compound().is_some() {
                    if i == last {
                        return Some(child.classname());
                    }
                    parent = Some(child.clone());
                    continue;
                }
                if child.as_leaf().is_some() {
                    return Some(child.classname());
                }
            }
        }
        None
    }

    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>> {
        let mut names: Vec<String> = vec![];
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));

        let depth = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.unwrap().by_index(*idx) {
                if let Some(name) = child.name() {
                    if child.as_compound().is_some() {
                        names.push(name.to_string());
                        parent = Some(child.clone());
                        continue;
                    }
                    if child.as_leaf().is_some() {
                        names.push(name.to_string());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                } else {
                    panic!("encountered unexpected inkMultiChildren at index {idx}");
                }
            }
            return None;
        }
        Some(names)
    }

    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>> {
        let mut indexes: Vec<usize> = vec![];
        let depth = path.len() - 1;
        let mut parent: Option<Widget> =
            Some(Widget::inkCanvasWidget(self.root_widget.data.clone()));
        for (i, name) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }

            if parent.as_ref().unwrap().is_leaf() {
                if i < depth {
                    return None;
                }
                break;
            }

            if let Some(compound) = parent.as_ref().unwrap().as_compound() {
                if let Some((idx, widget)) = compound.by_name(name) {
                    indexes.push(idx);
                    parent = Some(widget);
                    continue;
                }
            }
            return None;
        }
        Some(indexes)
    }
}

impl inkWidgetLibraryResource {
    pub fn root(&self) -> &inkWidgetLibraryItem {
        self.library_items.first().expect("Root")
    }
    pub fn root_chunk(&self) -> &inkWidgetLibraryItemInstance {
        &self.root().package.data.file.root_chunk
    }
}

impl WidgetTree for inkWidgetLibraryResource {
    fn get_widget_classname(&self, indexes: &[usize]) -> Option<String> {
        self.root_chunk().get_widget_classname(indexes)
    }

    fn get_path_names(&self, indexes: &[usize]) -> Option<Vec<String>> {
        self.root_chunk().get_path_names(indexes)
    }

    fn get_path_indexes(&self, names: &[&str]) -> Option<Vec<usize>> {
        self.root_chunk().get_path_indexes(names)
    }
}
