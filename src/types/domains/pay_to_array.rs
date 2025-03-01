use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::SchemaFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::KnownFormat;
use utoipa::PartialSchema;
use std::borrow::Cow;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::ToSchema;
use ethers::types::{H160, Address};
use serde::{Serialize, Deserialize};
use bytes::BytesMut;
use ethers::utils::to_checksum;
use std::{error::Error, str::FromStr};
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DomainPayToArray(pub Vec<Address>);

// Implement utoipa schema for OpenAPI documentation
/*
impl utoipa::PartialSchema for DomainPayToArray {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Array)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))
                .items(Some(Box::new(RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::AnyValue)
                        .build()
                )))))
                .build()
        ))
    }
}*/

impl utoipa::PartialSchema for DomainPayToArray {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type( SchemaType::Type(utoipa::openapi::Type::String ) )
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))

                /*.items(Some(Box::new(RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type( SchemaType::Type(utoipa::openapi::Type::Integer ) ) // Changed to String since U256 is represented as a string
                        .description(Some("U256 number as string"))
                        .build()
                )))))*/
                .build()
        ))
    }
}

impl utoipa::ToSchema for DomainPayToArray {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainPayToArray")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

// PostgreSQL serialization/deserialization
impl<'a> FromSql<'a> for DomainPayToArray {
   fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Parse the PostgreSQL array into a Vec<String> first
        let address_strings = <Vec<String> as FromSql>::from_sql(ty, raw)?;
        
        // Convert each string address to an ethers Address
        let mut addresses = Vec::new();
        for addr_str in address_strings {
            addresses.push(Address::from_str(&addr_str)?);
        }
        
        Ok(DomainPayToArray(addresses))
    }

    fn accepts(sql_type: &Type) -> bool {
        // Accept array of text/varchar types
        sql_type == &Type::TEXT_ARRAY || sql_type == &Type::VARCHAR_ARRAY
    }
}

impl ToSql for DomainPayToArray {
     fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert the Vec<Address> to Vec<String> with checksummed addresses
        let addresses: Vec<String> = self.0.iter()
            .map(|addr| format!("{}", to_checksum(addr, None)))
            .collect();
        
        // Use the PostgreSQL array serialization for Vec<String>
        <Vec<String> as ToSql>::to_sql(&addresses, ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::TEXT_ARRAY || sql_type == &Type::VARCHAR_ARRAY
    }

    to_sql_checked!();
}

// Helper methods
impl DomainPayToArray {
    pub fn to_string_array(&self) -> Vec<String> {
        self.0.iter()
            .map(|addr| format!("{:?}", addr))
            .collect()
    }
    
    pub fn new(addresses: Vec<Address>) -> Self {
        Self(addresses)
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// Allow easy conversion from Vec<Address> to DomainPayToArray
impl From<Vec<Address>> for DomainPayToArray {
    fn from(addresses: Vec<Address>) -> Self {
        Self(addresses)
    }
}

// Allow iteration over the addresses
impl AsRef<[Address]> for DomainPayToArray {
    fn as_ref(&self) -> &[Address] {
        &self.0
    }
}