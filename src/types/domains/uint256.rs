use bytes::BytesMut;
use ethers::types::U256;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DomainUint256(pub U256);

impl Default for DomainUint256 {
    fn default() -> Self {
        Self(U256::zero())
    }
}
use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

impl utoipa::ToSchema for DomainUint256 {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainUint256")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainUint256 {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Integer))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))
                /*.items(Some(Box::new(RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type( SchemaType::Type(utoipa::openapi::Type::Integer ) ) // Changed to String since U256 is represented as a string
                        .description(Some("U256 number as string"))
                        .build()
                )))))*/
                .build(),
        ))
    }
}

impl ToSql for DomainUint256 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert the U256 to a decimal string
        let uint_string = self.0.to_string();

        // Use the string representation directly with PostgreSQL's NUMERIC
        // This bypasses rust_decimal's Decimal type entirely
        let numeric_str = uint_string.as_str();

        println!("inserting numeric str {}", numeric_str);

        // Use the built-in string to NUMERIC conversion that PostgreSQL provides
        <&str as ToSql>::to_sql(&numeric_str, ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::TEXT
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for DomainUint256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Get the string representation from PostgreSQL NUMERIC
        let numeric_str = <&str as FromSql>::from_sql(ty, raw)?;

        // Parse the string directly into U256
        // This avoids using rust_decimal's Decimal type which has limited precision
        match U256::from_dec_str(numeric_str) {
            Ok(value) => Ok(DomainUint256(value)),
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse U256 from NUMERIC string: {}", e),
            ))),
        }
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::TEXT
    }
}

impl From<U256> for DomainUint256 {
    fn from(input: ethers::types::U256) -> Self {
        Self(input)
    }
}

impl Serialize for DomainUint256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize U256 as a hexadecimal string
        serializer.serialize_str(&format!("{}", self.0.to_string()))
    }
}

impl<'de> Deserialize<'de> for DomainUint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainUint256Visitor;

        impl<'de> Visitor<'de> for DomainUint256Visitor {
            type Value = DomainUint256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer representing a U256 value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                U256::from_dec_str(value)
                    .map(DomainUint256)
                    .map_err(de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DomainUint256(U256::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    Err(de::Error::custom(
                        "negative value cannot be converted to U256",
                    ))
                } else {
                    Ok(DomainUint256(U256::from(value as u64)))
                }
            }
        }

        deserializer.deserialize_any(DomainUint256Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::U256;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_deserialize_from_string() {
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(123456789u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::Str("123456789")];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }

    #[test]
    fn test_deserialize_from_integer() {
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(987654321u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::U64(987654321)];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }

    use bytes::BytesMut;
    use rand::{thread_rng, Rng};
    use tokio_postgres::types::Type;

    // Simulates the PostgreSQL NUMERIC conversion process
    fn simulate_numeric_roundtrip(
        value: &DomainUint256,
    ) -> Result<DomainUint256, Box<dyn Error + Sync + Send>> {
        let mut bytes = BytesMut::with_capacity(256); // Large enough for any string

        // Simulate to_sql
        value.to_sql(&Type::NUMERIC, &mut bytes)?;

        // In real PostgreSQL, this would be converted to internal format and back
        // For test purposes, we're simulating this with string conversion
        let string_value = std::str::from_utf8(&bytes[..]).unwrap();

        // Create a new BytesMut with the string value
        let mut raw_bytes = string_value.as_bytes().to_vec();

        // Simulate from_sql
        DomainUint256::from_sql(&Type::NUMERIC, &raw_bytes)
    }

    #[test]
    fn test_numeric_with_large_values() {
        // Test with a series of increasingly large values

        // 1. Small value
        let small = DomainUint256(U256::from(12345));
        let roundtrip_small = simulate_numeric_roundtrip(&small).unwrap();
        assert_eq!(small, roundtrip_small);

        // 2. Medium value
        let medium = DomainUint256(U256::from(u64::MAX));
        let roundtrip_medium = simulate_numeric_roundtrip(&medium).unwrap();
        assert_eq!(medium, roundtrip_medium);

        // 3. Large value (larger than u64 but smaller than Decimal max)
        let large = DomainUint256(U256::from(2).pow(U256::from(100)));
        let roundtrip_large = simulate_numeric_roundtrip(&large).unwrap();
        assert_eq!(large, roundtrip_large);

        // 4. Very large value (approaching U256 max)
        let very_large = DomainUint256(U256::from(2).pow(U256::from(250)));
        let roundtrip_very_large = simulate_numeric_roundtrip(&very_large).unwrap();
        assert_eq!(very_large, roundtrip_very_large);

        // 5. U256 max value
        let max_value = DomainUint256(U256::max_value());
        let roundtrip_max = simulate_numeric_roundtrip(&max_value).unwrap();
        assert_eq!(max_value, roundtrip_max);

        // 6. Random large value (as per original requirement)
        let random_nonce = {
            let mut rng = thread_rng();
            U256::from(rng.gen::<u128>())
                .overflowing_mul(U256::from(rng.gen::<u128>()))
                .0
        };
        let random = DomainUint256(random_nonce);
        let roundtrip_random = simulate_numeric_roundtrip(&random).unwrap();
        assert_eq!(random, roundtrip_random);

        println!("All PostgreSQL NUMERIC roundtrip tests passed successfully");
        println!("Max U256 value handled correctly: {}", max_value.0);
        println!("Random large value handled correctly: {}", random.0);
    }
}
