mod display;

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{CName, HDRColor, Vector2};

use super::InkWrapper;

mod implementation;

use super::conversion::deserialize_vector2_from_anything;

const OPACITY: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(None);
const FADEIN: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::In));
const FADEOUT: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::Out));

/// orphan interpolator
pub struct OrphanInkAnimInterpolator {
    pub index: usize,
    pub interpolator: InkWrapper<InkAnimInterpolator>,
}

/// transparency interpolation direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fade {
    /// transparency interpolates toward `1.`
    In,
    /// transparency interpolates toward `0.`
    Out,
}

/// every kind of possible interpolation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InkAnimInterpolatorType {
    Color,
    Size,
    Scale,
    Translation,
    Transparency(Option<Fade>),
    TextValueProgress,
    Effect,
    Anchor,
    Pivot,
    Shear,
    Rotation,
    Margin,
    Padding,
    TextReplace,
    TextOffset,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkanimInterpolationDirection)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    To = 0,
    From = 1,
    FromTo = 2,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkanimInterpolationMode)
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    EasyIn = 0,
    EasyOut = 1,
    EasyInOut = 2,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkanimInterpolationType)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Linear = 0,
    Quadratic = 1,
    Qubic = 2,
    Quartic = 3,
    Quintic = 4,
    Sinusoidal = 5,
    Exponential = 6,
    Elastic = 7,
    Circular = 8,
    Back = 9,
}

/// specific interpolator values interpretation
///
/// possible interpretations: percent-based (scale), positions-based (translation), color-based
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Range {
    Percent(f32),
    Position(Vector2),
    Color(HDRColor),
}

#[derive(Debug, Clone)]
pub struct Transformation {
    pub from: Range,
    pub to: Range,
}

/// generic interpolator
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interpolator {
    pub duration: f32,
    #[serde(deserialize_with = "deserialize_vector2_from_anything")]
    pub end_value: Range,
    pub interpolation_direction: Direction,
    pub interpolation_mode: Mode,
    pub interpolation_type: Type,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_additive: bool,
    pub start_delay: f32,
    #[serde(deserialize_with = "deserialize_vector2_from_anything")]
    pub start_value: Range,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_relative_duration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectInterpolator {
    pub effect_type: inkEffectType,
    pub effect_name: CName,
    pub param_name: CName,
    #[serde(flatten)]
    pub base: Interpolator,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum inkEffectType {
    ScanlineWipe = 0,
    LinearWipe = 1,
    RadialWipe = 2,
    LightSweep = 3,
    BoxBlur = 4,
    Mask = 5,
    Glitch = 6,
    PointCloud = 7,
    ColorFill = 8,
    InnerGlow = 9,
    ColorCorrection = 10,
    Multisampling = 11,
    Blackwall = 12,
}

/// any interpolator
///
/// possible kinds include: scale, translation, transparency, etc
#[allow(clippy::enum_variant_names)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum InkAnimInterpolator {
    inkanimScaleInterpolator(Interpolator),
    inkanimTranslationInterpolator(Interpolator),
    inkanimTransparencyInterpolator(Interpolator),
    inkanimSizeInterpolator(Interpolator),
    inkanimColorInterpolator(Interpolator),
    inkanimTextValueProgressInterpolator(Interpolator),
    inkanimEffectInterpolator(EffectInterpolator),
    inkanimAnchorInterpolator(Interpolator),
    inkanimPivotInterpolator(Interpolator),
    inkanimShearInterpolator(Interpolator),
    inkanimRotationInterpolator(Interpolator),
    inkanimMarginInterpolator(Interpolator),
    inkanimPaddingInterpolator(Interpolator),
    inkanimTextReplaceInterpolator(Interpolator),
    inkanimTextOffsetInterpolator(Interpolator),
}

impl AsRef<Interpolator> for InkAnimInterpolator {
    fn as_ref(&self) -> &Interpolator {
        match self {
            Self::inkanimEffectInterpolator(interpolator) => &interpolator.base,
            Self::inkanimScaleInterpolator(interpolator)
            | Self::inkanimTranslationInterpolator(interpolator)
            | Self::inkanimTransparencyInterpolator(interpolator)
            | Self::inkanimSizeInterpolator(interpolator)
            | Self::inkanimColorInterpolator(interpolator)
            | Self::inkanimTextValueProgressInterpolator(interpolator)
            | Self::inkanimAnchorInterpolator(interpolator)
            | Self::inkanimPivotInterpolator(interpolator)
            | Self::inkanimShearInterpolator(interpolator)
            | Self::inkanimRotationInterpolator(interpolator)
            | Self::inkanimMarginInterpolator(interpolator)
            | Self::inkanimPaddingInterpolator(interpolator)
            | Self::inkanimTextReplaceInterpolator(interpolator)
            | Self::inkanimTextOffsetInterpolator(interpolator) => interpolator,
        }
    }
}

impl InkAnimInterpolator {
    pub fn as_short_display(&self) -> &str {
        match self {
            Self::inkanimScaleInterpolator(_) => "scale",
            Self::inkanimTranslationInterpolator(_) => "translation",
            Self::inkanimTransparencyInterpolator(_) => "transparency",
            Self::inkanimSizeInterpolator(_) => "size",
            Self::inkanimColorInterpolator(_) => "color",
            Self::inkanimTextValueProgressInterpolator(_) => "text value progress",
            Self::inkanimEffectInterpolator(effect) => match effect.effect_type {
                inkEffectType::ScanlineWipe => "effect (scan line wipe)",
                inkEffectType::LinearWipe => "effect (linear wipe)",
                inkEffectType::RadialWipe => "effect (radial wipe)",
                inkEffectType::LightSweep => "effect (light sweep)",
                inkEffectType::BoxBlur => "effect (box blur)",
                inkEffectType::Mask => "effect (mask)",
                inkEffectType::Glitch => "effect (glitch)",
                inkEffectType::PointCloud => "effect (point cloud)",
                inkEffectType::ColorFill => "effect (color fill)",
                inkEffectType::InnerGlow => "effect (inner glow)",
                inkEffectType::ColorCorrection => "effect (color correction)",
                inkEffectType::Multisampling => "effect (multisampling)",
                inkEffectType::Blackwall => "effect (blackwall)",
            },
            Self::inkanimAnchorInterpolator(_) => "anchor",
            Self::inkanimPivotInterpolator(_) => "pivot",
            Self::inkanimShearInterpolator(_) => "shear",
            Self::inkanimRotationInterpolator(_) => "rotation",
            Self::inkanimMarginInterpolator(_) => "margin",
            Self::inkanimPaddingInterpolator(_) => "padding",
            Self::inkanimTextReplaceInterpolator(_) => "text replace",
            Self::inkanimTextOffsetInterpolator(_) => "text offset",
        }
    }
    pub fn starts(&self) -> f32 {
        self.as_ref().start_delay
    }
    pub fn ends(&self) -> f32 {
        self.starts() + self.as_ref().duration
    }
    pub fn direction(&self) -> Direction {
        self.as_ref().interpolation_direction
    }
    pub fn r#type(&self) -> Type {
        self.as_ref().interpolation_type
    }
    pub fn mode(&self) -> Mode {
        self.as_ref().interpolation_mode
    }
    pub fn duration(&self) -> f32 {
        self.as_ref().duration
    }
    pub fn transformation(&self) -> Transformation {
        Transformation {
            from: self.as_ref().start_value.clone(),
            to: self.as_ref().end_value.clone(),
        }
    }
}

impl PartialEq<InkAnimInterpolatorType> for InkAnimInterpolator {
    fn eq(&self, other: &InkAnimInterpolatorType) -> bool {
        match self {
            Self::inkanimScaleInterpolator(_) => other == &InkAnimInterpolatorType::Scale,
            Self::inkanimTranslationInterpolator(_) => {
                other == &InkAnimInterpolatorType::Translation
            }
            Self::inkanimTransparencyInterpolator(ref interpolator) => match other {
                InkAnimInterpolatorType::Transparency(None) => true,
                InkAnimInterpolatorType::Transparency(Some(Fade::In)) => {
                    interpolator.start_value < interpolator.end_value
                }
                InkAnimInterpolatorType::Transparency(Some(Fade::Out)) => {
                    interpolator.start_value > interpolator.end_value
                }
                _ => false,
            },
            Self::inkanimSizeInterpolator(_) => other == &InkAnimInterpolatorType::Size,
            Self::inkanimColorInterpolator(_) => other == &InkAnimInterpolatorType::Color,
            Self::inkanimTextValueProgressInterpolator(_) => {
                other == &InkAnimInterpolatorType::TextValueProgress
            }
            Self::inkanimEffectInterpolator(_) => other == &InkAnimInterpolatorType::Effect,
            Self::inkanimAnchorInterpolator(_) => other == &InkAnimInterpolatorType::Anchor,
            Self::inkanimPivotInterpolator(_) => other == &InkAnimInterpolatorType::Pivot,
            Self::inkanimShearInterpolator(_) => other == &InkAnimInterpolatorType::Shear,
            Self::inkanimRotationInterpolator(_) => other == &InkAnimInterpolatorType::Rotation,
            Self::inkanimMarginInterpolator(_) => other == &InkAnimInterpolatorType::Margin,
            Self::inkanimPaddingInterpolator(_) => other == &InkAnimInterpolatorType::Padding,
            Self::inkanimTextReplaceInterpolator(_) => {
                other == &InkAnimInterpolatorType::TextReplace
            }
            Self::inkanimTextOffsetInterpolator(_) => other == &InkAnimInterpolatorType::TextOffset,
        }
    }
}

/// a sequence of interpolators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimDefinition {
    pub interpolators: Vec<InkWrapper<InkAnimInterpolator>>,
}

/// a sequence of interpolations (interpolators and events)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequence {
    /// describe the interpolations played
    ///
    /// ⚠️ `definitions` size must always match `targets` size
    pub definitions: Vec<InkWrapper<InkAnimDefinition>>,
    pub name: CName,
    /// describe the targets onto which the interpolations are played
    ///
    /// ⚠️ `targets` size must always match `definitions` size
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimAnimationLibraryResource {
    pub sequences: Vec<InkWrapper<InkAnimSequence>>,
}

/// when related to interpolator(s),
/// corresponding target is a sequence of digits indicating the path to the nested element
///
/// see [NativeDB](https://nativedb.red4ext.com/inkanimSequenceTargetInfo)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequenceTargetInfo {
    /// path to the nested element (indexes)
    ///
    /// e.g. `[1,3,0,0,16]`
    pub path: Vec<usize>,
}

/// when declaring interpolation event(s), corresponding target has a negative handle ref ID
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlankInkAnimSequenceTargetInfo {
    /// typically here the value is `-1`
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub handle_ref_id: i32,
}

/// any target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    /// a sequence of digits (path to nested element) : when related to interpolator(s)
    WithHandleId(InkWrapper<InkAnimSequenceTargetInfo>),
    /// a negative [handle ID](super::HandleId) (not element related) : when declaring interpolation event(s)
    WithoutHandleId(BlankInkAnimSequenceTargetInfo),
}

impl InkAnimSequence {
    /// find all interpolators matching filter
    pub fn get_interpolators_matching(
        &self,
        filter: &InkAnimInterpolatorType,
    ) -> Vec<InkWrapper<InkAnimInterpolator>> {
        self.definitions
            .first()
            .expect("at least one ink anim definition")
            .data
            .interpolators
            .clone()
            .into_iter()
            .filter(|x| x.data == *filter)
            .collect()
    }
}

impl InkWrapper<InkAnimSequence> {
    pub fn name(&self) -> &str {
        self.data.name.as_str()
    }
}
