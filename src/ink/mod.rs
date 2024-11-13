use std::path::PathBuf;

use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
    ink::conversion::{deserialize_cname_from_format, deserialize_resourcepath_from_format},
    RedsValue, RedsWidget,
};

use self::{
    anim::{InkAnimSequence, Target},
    widget::{inkMultiChildren, SiblingOrNested, Widget},
};
mod conversion;
use conversion::deserialize_lockey_from_anything;

/// everything related to *.inkanim*
pub mod anim;
/// everything related to *.inkwidget*
pub mod widget;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Storage {
    #[default]
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcePath(
    #[serde(deserialize_with = "deserialize_resourcepath_from_format")] pub std::path::PathBuf,
);

impl Default for ResourcePath {
    fn default() -> Self {
        Self(PathBuf::new())
    }
}

impl RedsValue for ResourcePath {
    fn reds_value(&self) -> String {
        format!("r\"{}\"", self.0.as_os_str().to_str().unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data<T> {
    pub version: usize,
    pub build_version: usize,
    pub root_chunk: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    wolven_kit_version: String,
    w_kit_json_version: String,
    game_version: usize,
    exported_date_time: chrono::DateTime<chrono::Utc>,
    data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct File<T> {
    pub header: Header,
    pub data: Data<T>,
}

/// see [NativeDB](https://nativedb.red4ext.com/Vector2)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, RedsValue)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl red4ext_rs::NativeRepr for Vector2 {
    const NAME: &'static str = "Vector2";
}

/// see [NativeDB](https://nativedb.red4ext.com/HDRColor)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct HDRColor {
    pub alpha: f32,
    pub blue: f32,
    pub green: f32,
    pub red: f32,
}

unsafe impl red4ext_rs::NativeRepr for HDRColor {
    const NAME: &'static str = "HDRColor";
}

/// asset handle ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, RedsValue)]
#[serde(transparent)]
pub struct HandleId(#[serde(deserialize_with = "deserialize_number_from_string")] u32);

unsafe impl red4ext_rs::NativeRepr for HandleId {
    const NAME: &'static str = "HandleId";
}

#[cfg(test)]
impl From<u32> for HandleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// wrapper with handle ID
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    pub handle_id: HandleId,
    pub data: T,
}

impl InkWrapper<inkMultiChildren> {
    pub fn iter(&self) -> std::slice::Iter<'_, InkWrapper<Widget>> {
        self.data.iter()
    }
}

impl<T> RedsWidget for InkWrapper<T>
where
    T: RedsWidget,
{
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        self.data.reds_widget(name, parent)
    }
}

/// specific resource ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CName(#[serde(deserialize_with = "deserialize_cname_from_format")] pub String);

impl Default for CName {
    fn default() -> Self {
        Self("None".to_string())
    }
}

impl CName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl RedsValue for CName {
    fn reds_value(&self) -> String {
        format!("n\"{}\"", self.0.clone())
    }
}

unsafe impl red4ext_rs::NativeRepr for CName {
    const NAME: &'static str = "CName";
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocKey {
    ID(u32),
    Value(String),
}

/// specific translation ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocalizationString {
    #[serde(deserialize_with = "deserialize_lockey_from_anything")]
    pub value: Option<LocKey>,
}

unsafe impl red4ext_rs::NativeRepr for LocalizationString {
    const NAME: &'static str = "LocalizationString";
}

impl<T> std::fmt::Display for InkWrapper<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.handle_id, self.data)
    }
}

impl std::fmt::Display for HandleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🔑 {}", self.0)
    }
}

/// animation aggregated informations summary
#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct PathSummary {
    /// animation name
    Name: CName,
    /// unique handle ID
    HandleId: HandleId,
    /// index in sequence
    Index: usize,
    /// path to the nested element
    Path: Vec<usize>,
}

impl InkAnimSequence {
    /// summarize all paths matching sequences of digits
    pub fn get_path_indexes_matching(&self, searched: &[usize]) -> Vec<PathSummary> {
        let count = searched.len();
        let _last = count - 1;
        let mut out = vec![];
        for (target_index, target) in self.targets.iter().enumerate() {
            match target {
                Target::WithHandleId(ref handle) => {
                    let path = &handle.data.path;
                    if path.sibling_or_nested(searched) {
                        out.push(PathSummary {
                            Name: self.name.clone(),
                            HandleId: handle.handle_id,
                            Index: target_index,
                            Path: path.clone(),
                        });
                    }
                }
                _ => continue,
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use inkanim_macros::RedsWidgetLeaf;

    use crate::{
        widget::{inkCanvasWidget, inkMultiChildren, inkTextWidget, layout::inkEHorizontalAlign},
        InkWrapper, RedsWidgetCompound, RedsWidgetLeaf, Vector2,
    };
    #[derive(RedsWidgetLeaf, Debug, Clone, Default, PartialEq)]
    pub struct TestChild {
        pub content_h_align: inkEHorizontalAlign,
        pub size: Vector2,
    }
    unsafe impl red4ext_rs::NativeRepr for TestChild {
        const NAME: &'static str = "TChild";
    }
    #[test]
    fn reds_default() {
        let child = TestChild {
            content_h_align: inkEHorizontalAlign::Fill,
            size: Vector2 { x: 0., y: 0. },
        };
        assert_eq!(
            child.reds_widget_leaf("element", None),
            r#"let element = new TChild();"#
        );
    }
    #[test]
    fn reds_simple() {
        let child = TestChild {
            content_h_align: inkEHorizontalAlign::Center,
            size: Vector2 { x: 1., y: 0.6 },
        };
        assert_eq!(
            child.reds_widget_leaf("element", None),
            r#"let element = new TChild();
element.content_h_align = inkEHorizontalAlign.Center;
element.size = new Vector2(1., 0.6);"#
        );
    }
    #[test]
    fn reds_tree() {
        let inner = inkTextWidget {
            name: crate::CName("shape".to_string()),
            layout: crate::widget::layout::inkWidgetLayout {
                ..Default::default()
            },
            property_manager: None,
            render_transform_pivot: Vector2 { x: 1., y: 3. },
            render_transform: crate::widget::layout::inkUITransform {
                ..Default::default()
            },
            size: Vector2 { x: 360., y: 100. },
            ..Default::default()
        };
        let child = crate::widget::Widget::inkTextWidget(inner.clone());
        let parent = inkCanvasWidget {
            children: InkWrapper {
                handle_id: 1.into(),
                data: inkMultiChildren {
                    children: vec![InkWrapper {
                        handle_id: 2.into(),
                        data: child,
                    }],
                },
            },
            name: crate::CName("main_canvas".to_string()),
            child_order: crate::widget::layout::inkEChildOrder::Backward,
            child_margin: crate::widget::layout::inkMargin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
        };
        assert_eq!(
            inner.reds_widget_leaf("element", Some("parent")),
            r#"let element = new inkText();
element.name = n"shape";
element.render_transform_pivot = new Vector2(1., 3.);
element.size = new Vector2(360., 100.);
element.line_height_percentage = 0.;
element.scroll_delay = 0;
element.scroll_text_speed = 0.;"#
        );
        assert_eq!(
            parent.reds_widget_compound("parent", None),
            r#"let parent = new inkCanvas();
parent.name = n"main_canvas";
parent.child_order = inkEChildOrder.Backward;
let shape = new inkText();
shape.name = n"shape";
shape.render_transform_pivot = new Vector2(1., 3.);
shape.size = new Vector2(360., 100.);
shape.line_height_percentage = 0.;
shape.scroll_delay = 0;
shape.scroll_text_speed = 0.;
parent.AddChild(shape);"#
        );
    }
}
