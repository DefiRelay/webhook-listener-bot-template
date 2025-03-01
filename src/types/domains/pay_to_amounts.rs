use serde::de;
use serde::Serializer;

 

use serde::Deserializer;
use serde::de::Visitor;
use std::fmt;
use ethers::types::U256;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::SchemaFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::KnownFormat;
use utoipa::PartialSchema;
use std::borrow::Cow;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use bytes::BytesMut;
use rust_decimal::Decimal;
use std::{error::Error, str::FromStr};
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};


/*

Seems to have some sort of  hex - to -decimal issue 


*/
 

#[derive(Debug, Clone, Eq, PartialEq )]
pub struct DomainPayToAmounts(pub Vec<U256>);

// Implement utoipa schema for OpenAPI documentation   --- this is wrong ? 
impl utoipa::PartialSchema for DomainPayToAmounts {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type( SchemaType::Type(utoipa::openapi::Type::Integer ) )
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

impl utoipa::ToSchema for DomainPayToAmounts {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainPayToAmounts")
    }
    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

// Implement ToSql for U256
/*
impl ToSql for U256 {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert U256 to Decimal for storage
        let decimal_str = self.to_string();
        let decimal = Decimal::from_str(&decimal_str)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)?;
        
        // Use Decimal's ToSql implementation
        decimal.to_sql(ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC
    }

    to_sql_checked!();
}

// Implement FromSql for U256
impl<'a> FromSql<'a> for U256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // First, convert from SQL to Decimal
        let decimal = Decimal::from_sql(ty, raw)?;
        
        // Then convert Decimal to U256
        let decimal_str = decimal.to_string();
        U256::from_dec_str(&decimal_str)
            .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC
    }
}*/
// PostgreSQL serialization/deserialization for DomainPayToAmounts
impl<'a> FromSql<'a> for DomainPayToAmounts {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Parse the PostgreSQL array directly into a Vec<Decimal>
        let decimal_amounts = <Vec<Decimal> as FromSql>::from_sql(ty, raw)?;
        
        // Convert Vec<Decimal> to Vec<U256>
        let u256_amounts = decimal_amounts
            .into_iter()
            .map(|decimal| {
                let decimal_str = decimal.to_string();
                U256::from_dec_str(&decimal_str)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)
            })
            .collect::<Result<Vec<U256>, Box<dyn Error + Sync + Send>>>()?;
        
        Ok(DomainPayToAmounts(u256_amounts))
    }
    
    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC_ARRAY
    }
}

impl ToSql for DomainPayToAmounts {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert Vec<U256> to Vec<Decimal>
        let decimal_amounts: Vec<Decimal> = self.0
            .iter()
            .map(|u256| {

                let num_str = u256.to_string();
                Decimal::from_str(&num_str)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)
            })
            .collect::<Result<Vec<Decimal>, Box<dyn Error + Sync + Send>>>()?;
        
        // Use the PostgreSQL array serialization for Vec<Decimal>
        <Vec<Decimal> as ToSql>::to_sql(&decimal_amounts, ty, out)
    }
    
    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC_ARRAY
    }
    
    to_sql_checked!();
}


// Helper methods
impl DomainPayToAmounts {
    pub fn to_string_array(&self) -> Vec<String> {
        self.0.iter()
            .map(|amount| amount.to_string())
            .collect()
    }
    
    pub fn new(amounts: Vec<U256>) -> Self {
        Self(amounts)
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn sum(&self) -> U256 {
        self.0.iter().fold(U256::zero(), |acc, x| acc + *x)
    }
}

// Allow easy conversion from Vec<U256> to DomainPayToAmounts
impl From<Vec<U256>> for DomainPayToAmounts {
    fn from(amounts: Vec<U256>) -> Self {
        Self(amounts)
    }
}

// Allow iteration over the amounts
impl AsRef<[U256]> for DomainPayToAmounts {
    fn as_ref(&self) -> &[U256] {
        &self.0
    }
}





 
 
impl Serialize for DomainPayToAmounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a vector of strings where each U256 is represented as a decimal string
        let string_values: Vec<String> = self.0
            .iter()
            .map(|amount| amount.to_string())
            .collect();
        
        string_values.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DomainPayToAmounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainPayToAmountsVisitor;
        
        impl<'de> Visitor<'de> for DomainPayToAmountsVisitor {
            type Value = DomainPayToAmounts;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of strings or numbers representing U256 values")
            }
            
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut amounts = Vec::new();
                
                // Process each element in the sequence
                while let Some(value) = seq.next_element::<serde_json::Value>()? {
                    match value {
                        serde_json::Value::String(s) => {
                            let amount = U256::from_dec_str(&s)
                                .map_err(|e| de::Error::custom(format!("Invalid U256 string: {}", e)))?;
                            amounts.push(amount);
                        },
                        serde_json::Value::Number(n) => {
                            if let Some(n_u64) = n.as_u64() {
                                amounts.push(U256::from(n_u64));
                            } else {
                                return Err(de::Error::custom("Number too large for u64 or negative"));
                            }
                        },
                        _ => return Err(de::Error::custom("Expected string or number for U256")),
                    }
                }
                
                Ok(DomainPayToAmounts(amounts))
            }
        }
        
        deserializer.deserialize_seq(DomainPayToAmountsVisitor)
    }
}

/*

// Helper method to convert to string array (useful for API responses)
impl DomainPayToAmounts {
    pub fn to_string_array(&self) -> Vec<String> {
        self.0.iter().map(|amt| amt.to_string()).collect()
    }
}*/