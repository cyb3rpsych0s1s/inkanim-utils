//! .inkwidget
//!
//! All the widgets in Cybperunk 2077 UI
//! are similar to the web and traditional UI frameworks.

pub mod font;
pub mod image;
pub(crate) mod implementation;
pub mod layout;
pub mod properties;

use enum_dispatch::enum_dispatch;
pub use implementation::*;

use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_bool_from_anything;

use crate::{CName, RedsWidget, ResourcePath};

use self::{
    font::{
        fontStyle, inkFontFamilyResource, textHorizontalAlignment, textLetterCase,
        textOverflowPolicy, textVerticalAlignment,
    },
    image::{inkBrushMirrorType, inkBrushTileType, inkTextureAtlas},
    layout::{inkEHorizontalAlign, inkEVerticalAlign, inkMargin, textJustificationType},
};

use super::{HandleId, InkWrapper, LocalizationString};

/// belongs to the same level or is nested below, in a tree hierarchy
pub trait SiblingOrNested {
    fn sibling_or_nested(&self, searched: &[usize]) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, RedsValue, PartialEq)]
#[non_exhaustive]
pub enum Flags {
    #[default]
    Default,
    Soft,
    Hard,
}

unsafe impl red4ext_rs::NativeRepr for Flags {
    const NAME: &'static str = "Flags";
}

macro_rules! native_compound_widget {
    ($ty:ident) => {
        #[doc=concat!("see [NativeDB](https://nativedb.red4ext.com/", stringify!($ty), ")")]
        #[allow(non_camel_case_types)]
        #[derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Default,
            PartialEq,
            inkanim_macros::RedsWidgetCompound,
        )]
        #[serde(rename_all = "camelCase")]
        pub struct $ty {
            pub children: InkWrapper<inkMultiChildren>,
            pub name: $crate::CName,
            pub child_order: self::layout::inkEChildOrder,
            pub child_margin: self::layout::inkMargin,
        }
        unsafe impl red4ext_rs::NativeRepr for $ty {
            const NAME: &'static str = ::const_str::replace!(::std::stringify!($ty), "Widget", "");
        }
    };
}

macro_rules! native_leaf_widget {
    ($ty:ident { $($tt:tt)* }) => {
        #[doc=concat!("🌿 see [NativeDB](https://nativedb.red4ext.com/", stringify!($ty), ")")]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, inkanim_macros::RedsWidgetLeaf)]
        #[serde(rename_all = "camelCase")]
        pub struct $ty {
            pub name: $crate::CName,
            pub layout: self::layout::inkWidgetLayout,
            pub property_manager: Option<self::properties::PropertyManager>,
            pub render_transform_pivot: crate::Vector2,
            pub render_transform: self::layout::inkUITransform,
            pub size: crate::Vector2,
            $($tt)*
        }
        unsafe impl red4ext_rs::NativeRepr for $ty {
            const NAME: &'static str = ::const_str::replace!(::std::stringify!($ty), "Widget", "");
        }
    };
    ($ty:ident) => {
        native_leaf_widget!($ty {});
    };
}

native_compound_widget!(inkCanvasWidget);
native_compound_widget!(inkHorizontalPanelWidget);
native_compound_widget!(inkVerticalPanelWidget);
native_compound_widget!(inkScrollAreaWidget);
native_compound_widget!(inkUniformGridWidget);
native_compound_widget!(inkVirtualCompoundWidget);
native_compound_widget!(inkFlexWidget);
native_compound_widget!(inkCacheWidget);

/// see [NativeDB](https://nativedb.red4ext.com/inkMultiChildren)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct inkMultiChildren {
    pub children: Vec<InkWrapper<Widget>>,
}

impl RedsWidget for inkMultiChildren {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        self.children
            .iter()
            .map(|x| x.reds_widget(name, parent))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl inkMultiChildren {
    pub fn iter(&self) -> std::slice::Iter<'_, InkWrapper<Widget>> {
        self.children.iter()
    }
}

native_leaf_widget!(inkTextWidget {
  pub localization_string: LocalizationString,
  pub text: String,
  pub font_family: inkFontFamilyResource,
  pub font_style: fontStyle,
  pub justification: textJustificationType,
  pub text_letter_case: Option<textLetterCase>,
  pub line_height_percentage: f32,
  pub text_horizontal_alignment: textHorizontalAlignment,
  pub text_vertical_alignment: textVerticalAlignment,
  pub text_overflow_policy: textOverflowPolicy,
  pub content_h_align: inkEHorizontalAlign,
  pub content_v_align: inkEVerticalAlign,
  pub scroll_delay: u16,
  pub scroll_text_speed: f32,
});
native_leaf_widget!(inkImageWidget {
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_external_dynamic_texture: bool,
    pub external_dynamic_texture: CName,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_nine_slice_scale: bool,
    pub nine_slice_scale: inkMargin,
    pub mirror_type: inkBrushMirrorType,
    pub tile_type: inkBrushTileType,
    pub horizontal_tile_crop: f32,
    pub vertical_tile_crop: f32,
    pub texture_atlas: inkTextureAtlas,
    pub texture_part: CName,
    pub content_h_align: inkEHorizontalAlign,
    pub content_v_align: inkEVerticalAlign,
    pub tile_h_align: inkEHorizontalAlign,
    pub tile_v_align: inkEVerticalAlign,
});
native_leaf_widget!(inkVideoWidget);
native_leaf_widget!(inkMaskWidget);
native_leaf_widget!(inkBorderWidget);
native_leaf_widget!(inkShapeWidget);
native_leaf_widget!(inkCircleWidget);
native_leaf_widget!(inkRectangleWidget);
native_leaf_widget!(inkVectorGraphicWidget);

/// any widget
#[allow(clippy::enum_variant_names)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[enum_dispatch(Classname)]
#[serde(tag = "$type")]
pub enum Widget {
    inkMultiChildren(inkMultiChildren),

    inkCanvasWidget(inkCanvasWidget),
    inkHorizontalPanelWidget(inkHorizontalPanelWidget),
    inkVerticalPanelWidget(inkVerticalPanelWidget),
    inkScrollAreaWidget(inkScrollAreaWidget),
    inkUniformGridWidget(inkUniformGridWidget),
    inkVirtualCompoundWidget(inkVirtualCompoundWidget),
    inkFlexWidget(inkFlexWidget),
    inkCacheWidget(inkCacheWidget),

    inkTextWidget(inkTextWidget),
    inkImageWidget(inkImageWidget),
    inkVideoWidget(inkVideoWidget),
    inkMaskWidget(inkMaskWidget),
    inkBorderWidget(inkBorderWidget),
    inkShapeWidget(inkShapeWidget),
    inkCircleWidget(inkCircleWidget),
    inkRectangleWidget(inkRectangleWidget),
    inkVectorGraphicWidget(inkVectorGraphicWidget),
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryItemInstance)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryItemInstance {
    pub root_widget: InkWrapper<inkCanvasWidget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub file: crate::Data<inkWidgetLibraryItemInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    pub data: self::Data,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryItem)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct inkWidgetLibraryItem {
    pub name: CName,
    pub package: Package,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkanimAnimationLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimAnimationLibraryResource {
    depot_path: ResourcePath,
    flags: Flags,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryResource {
    pub animation_library_res_ref: inkanimAnimationLibraryResource,
    pub library_items: Vec<inkWidgetLibraryItem>,
}

/// widget aggregated informations summary
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSummary {
    /// unique handle ID
    pub HandleId: HandleId,
    /// widget name
    pub Name: CName,
}
