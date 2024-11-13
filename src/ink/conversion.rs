use std::fmt;

use serde::de::{self, MapAccess};
use serde::Deserialize;

use crate::anim::Range;
use crate::LocKey;

pub fn deserialize_cname_from_format<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct CNameVisitor;
    impl<'de> de::Visitor<'de> for CNameVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("$type CName, with valid $storage and $value")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut type_ok = false;
            let mut storage_ok = false;
            let mut value_ok = false;
            let mut value: String = String::new();
            while let Some(key) = map.next_key::<&str>()? {
                if key == "$type" {
                    let value: &str = map.next_value()?;
                    if value != "CName" {
                        return Err(de::Error::custom("invalid map type"));
                    } else {
                        type_ok = true;
                    }
                }
                if key == "$storage" {
                    let value: &str = map.next_value()?;
                    if value != "string" {
                        return Err(de::Error::custom("invalid map storage"));
                    } else {
                        storage_ok = true;
                    }
                }
                if key == "$value" {
                    value = map.next_value::<String>()?;
                    value_ok = true;
                }
            }
            if type_ok && storage_ok && value_ok {
                return Ok(value);
            }
            Err(de::Error::custom("invalid map sequence"))
        }
    }
    deserializer.deserialize_any(CNameVisitor)
}

pub fn deserialize_resourcepath_from_format<'de, D>(
    deserializer: D,
) -> Result<std::path::PathBuf, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct ResourcePathVisitor;
    impl<'de> de::Visitor<'de> for ResourcePathVisitor {
        type Value = std::path::PathBuf;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("$type ResourcePath, with valid $storage and $value")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut type_ok = false;
            let mut storage_ok = false;
            let mut value_ok = false;
            let mut value: String = String::new();
            while let Some(key) = map.next_key::<&str>()? {
                if key == "$type" {
                    let value: &str = map.next_value()?;
                    if value != "ResourcePath" {
                        return Err(de::Error::custom("invalid map type"));
                    } else {
                        type_ok = true;
                    }
                }
                if key == "$storage" {
                    let value: &str = map.next_value()?;
                    if value != "string" {
                        return Err(de::Error::custom("invalid map storage"));
                    } else {
                        storage_ok = true;
                    }
                }
                if key == "$value" {
                    value = map.next_value::<String>()?;
                    value_ok = true;
                }
            }
            if type_ok && storage_ok && value_ok {
                return Ok(std::path::PathBuf::from(value));
            }
            Err(de::Error::custom("invalid map sequence"))
        }
    }
    deserializer.deserialize_any(ResourcePathVisitor)
}

pub fn deserialize_lockey_from_anything<'de, D>(deserializer: D) -> Result<Option<LocKey>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct LocKeyVisitor;
    /// length of numeric ID
    const ID_LEN: usize = 5;
    /// maximum numeric ID
    const ID_MAX: usize = 99999;

    impl<'de> de::Visitor<'de> for LocKeyVisitor {
        type Value = Option<LocKey>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("either a String, or its simpler integer representation")
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v > ID_MAX as u32 {
                return Err(de::Error::custom("greater than 5 digits"));
            }
            Ok(Some(LocKey::ID(v)))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v < 0 {
                return Err(de::Error::custom("negative digit"));
            }
            self.visit_u32(v as u32)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Ok(Some(LocKey::Value(String::new())));
            }
            let searched = "LocKey#";
            let upper = searched.len();
            let expected = upper + ID_LEN;
            if v.len() == expected && &v[0..upper] == searched {
                let id = v[upper..].parse::<u32>().map_err(|_| {
                    de::Error::custom(format!("unexpected loc key ID: {}", &v[upper..]))
                })?;
                return Ok(Some(LocKey::ID(id)));
            }
            Ok(Some(LocKey::Value(v.to_string())))
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            while let Some(key) = map.next_key::<&str>()? {
                if key == "value" {
                    let value: &str = map.next_value()?;
                    return self.visit_str(value);
                }
            }
            Err(de::Error::custom("invalid map sequence"))
        }
    }
    deserializer.deserialize_any(LocKeyVisitor)
}

pub fn deserialize_vector2_from_anything<'de, D>(deserializer: D) -> Result<Range, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct RangeVisitor;

    impl<'de> de::Visitor<'de> for RangeVisitor {
        type Value = Range;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "either a Vector2, HDRColor, or its simpler integer or float representation",
            )
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Percent(v as f32))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Percent(v))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_f32(v as f32)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let (key, value) = &mut map.next_entry::<&str, &str>()?.unwrap();
            if key == &"$type" {
                if value == &"Vector2" {
                    return Ok(Range::Position(crate::Vector2::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
                if value == &"HDRColor" {
                    return Ok(Range::Color(crate::HDRColor::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
            }
            Err(de::Error::custom("unknown type"))
        }
    }

    deserializer.deserialize_any(RangeVisitor)
}
