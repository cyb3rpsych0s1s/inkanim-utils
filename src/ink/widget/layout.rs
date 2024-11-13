use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};

use crate::Vector2;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum inkEAnchor {
    #[default]
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    CenterLeft = 3,
    Centered = 4,
    CenterRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
    TopFillHorizontaly = 9,
    CenterFillHorizontaly = 10,
    BottomFillHorizontaly = 11,
    LeftFillVerticaly = 12,
    CenterFillVerticaly = 13,
    RightFillVerticaly = 14,
    Fill = 15,
}

unsafe impl red4ext_rs::NativeRepr for inkEAnchor {
    const NAME: &'static str = "inkEAnchor";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, RedsValue)]
pub enum inkEHorizontalAlign {
    #[default]
    Fill = 0,
    Left = 1,
    Center = 2,
    Right = 3,
}

unsafe impl red4ext_rs::NativeRepr for inkEHorizontalAlign {
    const NAME: &'static str = "inkEHorizontalAlign";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, RedsValue)]
pub enum inkEVerticalAlign {
    #[default]
    Fill = 0,
    Top = 1,
    Center = 2,
    Bottom = 3,
}

unsafe impl red4ext_rs::NativeRepr for inkEVerticalAlign {
    const NAME: &'static str = "inkEVerticalAlign";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub struct inkUITransform {
    pub translation: Vector2,
    pub scale: Vector2,
    pub shear: Vector2,
    pub rotation: f32,
}

unsafe impl red4ext_rs::NativeRepr for inkUITransform {
    const NAME: &'static str = "inkUITransform";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum textJustificationType {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

unsafe impl red4ext_rs::NativeRepr for textJustificationType {
    const NAME: &'static str = "textJustificationType";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
#[serde(tag = "$type")]
pub struct inkMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

unsafe impl red4ext_rs::NativeRepr for inkMargin {
    const NAME: &'static str = "inkMargin";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub struct inkWidgetLayout {
    pub padding: inkMargin,
    pub margin: inkMargin,
    pub anchor_point: Vector2,
    pub size_coefficient: f32,
    #[serde(rename = "HAlign")]
    pub h_align: inkEHorizontalAlign,
    #[serde(rename = "VAlign")]
    pub v_align: inkEVerticalAlign,
    pub anchor: inkEAnchor,
    pub size_rule: inkESizeRule,
}

unsafe impl red4ext_rs::NativeRepr for inkWidgetLayout {
    const NAME: &'static str = "inkWidgetLayout";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum inkEChildOrder {
    #[default]
    Forward = 0,
    Backward = 1,
}

unsafe impl red4ext_rs::NativeRepr for inkEChildOrder {
    const NAME: &'static str = "inkEChildOrder";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum inkESizeRule {
    #[default]
    Fixed = 0,
    Stretch = 1,
}

unsafe impl red4ext_rs::NativeRepr for inkESizeRule {
    const NAME: &'static str = "inkESizeRule";
}
