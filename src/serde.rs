use core::str::FromStr;

use crate::{ByteSizeIec, ByteSizeSi};

impl serde::Serialize for ByteSizeSi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for ByteSizeSi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::Unexpected;
        use serde::de::Visitor;

        struct ByteSizeSiVistor;

        impl<'de> Visitor<'de> for ByteSizeSiVistor {
            type Value = ByteSizeSi;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("an unsigned number with byte unit")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v), &self))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Other("negative integer"), &self))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeSi::b(v))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeSi::b(v))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeSi::b(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeSi(v))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeSi)
                    .map_err(|_| Error::invalid_value(Unexpected::Other("overflow integer"), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                ByteSizeSi::from_str(v).map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(ByteSizeSiVistor)
        } else {
            deserializer.deserialize_u64(ByteSizeSiVistor)
        }
    }
}

impl serde::Serialize for ByteSizeIec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for ByteSizeIec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::Unexpected;
        use serde::de::Visitor;

        struct ByteSizeIecVistor;

        impl<'de> Visitor<'de> for ByteSizeIecVistor {
            type Value = ByteSizeIec;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("an unsigned number with byte unit")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v as i64), &self))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Signed(v), &self))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Other("negative integer"), &self))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeIec::b(v))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeIec::b(v))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeIec::b(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ByteSizeIec(v))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map(ByteSizeIec)
                    .map_err(|_| Error::invalid_value(Unexpected::Other("overflow integer"), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                ByteSizeIec::from_str(v)
                    .map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(ByteSizeIecVistor)
        } else {
            deserializer.deserialize_u64(ByteSizeIecVistor)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{ByteSizeIec, ByteSizeSi};

    #[test]
    fn test_ser_si() {
        let v = serde_json::to_value(ByteSizeSi::kb(18666)).unwrap();
        assert_eq!(v, json!("18.7MB"));
    }

    #[test]
    fn test_ser_iec() {
        let v = serde_json::to_value(ByteSizeIec::kib(7987)).unwrap();
        assert_eq!(v, json!("7.8MiB"));
    }

    #[test]
    fn test_deser_si() {
        let v: ByteSizeSi = serde_json::from_value(json!(114514)).unwrap();
        assert_eq!(v, ByteSizeSi(114514));

        let v: ByteSizeSi = serde_json::from_value(json!("114514")).unwrap();
        assert_eq!(v, ByteSizeSi(114514));

        let v: ByteSizeSi = serde_json::from_value(json!("1.1MB")).unwrap();
        assert_eq!(v, ByteSizeSi::kb(1100));
    }

    #[test]
    fn test_deser_iec() {
        let v: ByteSizeIec = serde_json::from_value(json!(114514)).unwrap();
        assert_eq!(v, ByteSizeIec(114514));

        let v: ByteSizeIec = serde_json::from_value(json!("114514")).unwrap();
        assert_eq!(v, ByteSizeIec(114514));

        let v: ByteSizeIec = serde_json::from_value(json!("1.5GiB")).unwrap();
        assert_eq!(v, ByteSizeIec::mib(1536));
    }
}
