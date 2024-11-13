use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};

use crate::{CName, ResourcePath};

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct inkFontFamilyResource {
    depot_path: ResourcePath,
    flags: Flags,
}

unsafe impl red4ext_rs::NativeRepr for inkFontFamilyResource {
    const NAME: &'static str = "inkFontFamilyResource";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
pub struct fontStyle(pub CName);

unsafe impl red4ext_rs::NativeRepr for fontStyle {
    const NAME: &'static str = "fontStyle";
}

#[allow(non_camel_case_types, clippy::enum_variant_names)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
pub enum textLetterCase {
    #[default]
    OriginalCase = 0,
    UpperCase = 1,
    LowerCase = 2,
}

unsafe impl red4ext_rs::NativeRepr for textLetterCase {
    const NAME: &'static str = "textLetterCase";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
pub enum textHorizontalAlignment {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

unsafe impl red4ext_rs::NativeRepr for textHorizontalAlignment {
    const NAME: &'static str = "textHorizontalAlignment";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
pub enum textVerticalAlignment {
    #[default]
    Top = 0,
    Center = 1,
    Bottom = 2,
}

unsafe impl red4ext_rs::NativeRepr for textVerticalAlignment {
    const NAME: &'static str = "textVerticalAlignment";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize, Default, RedsValue, PartialEq)]
pub enum textOverflowPolicy {
    #[default]
    None = 0,
    DotsEnd = 1,
    DotsEndLastLine = 2,
    AutoScroll = 3,
    PingPongScroll = 4,
    AdjustToSize = 5,
}

unsafe impl red4ext_rs::NativeRepr for textOverflowPolicy {
    const NAME: &'static str = "textOverflowPolicy";
}
