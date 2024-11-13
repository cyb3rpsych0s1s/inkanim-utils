use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};

use crate::{CName, HandleId};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, RedsValue)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub struct inkPropertyBinding {
    pub property_name: CName,
    pub style_path: CName,
}

unsafe impl red4ext_rs::NativeRepr for inkPropertyBinding {
    const NAME: &'static str = "inkPropertyBinding";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, RedsValue)]
#[serde(tag = "$type")]
pub struct inkPropertyManager {
    pub bindings: Vec<inkPropertyBinding>,
}

unsafe impl red4ext_rs::NativeRepr for inkPropertyManager {
    const NAME: &'static str = "inkPropertyManager";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, RedsValue)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyManager {
    pub handle_id: HandleId,
    pub data: inkPropertyManager,
}

unsafe impl red4ext_rs::NativeRepr for PropertyManager {
    const NAME: &'static str = "PropertyManager";
}
