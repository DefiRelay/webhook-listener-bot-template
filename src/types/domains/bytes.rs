

use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::SchemaFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::KnownFormat;
use utoipa::PartialSchema;
use std::borrow::Cow;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::ToSchema;


use serde::Serialize;
use serde::Deserialize;
use bytes::BytesMut;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};


#[derive(Debug,Clone,Eq,PartialEq,Serialize,Deserialize)]
pub struct DomainBytes(pub Vec<u8>);


   // ???
impl utoipa::PartialSchema for DomainBytes {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type( SchemaType:: AnyValue )  // Use string type for hexadecimal address
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))  // 'byte' format for base64 encoding, adjust if you have a 'hex' option
                .build()
        ))
    }
}


impl utoipa::ToSchema for DomainBytes {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainEthAddress")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}




impl<'a> FromSql<'a> for DomainBytes {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(DomainBytes(raw.to_vec()))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::BYTEA
    }
}

impl ToSql for DomainBytes {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(&self.0);
        Ok(IsNull::No)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::BYTEA
    }

    to_sql_checked!();
}