#[macro_export]
macro_rules! prep_json_field {
    ($field:ty) => {
        impl From<$field> for sea_orm::Value {
            fn from(model: $field) -> Self {
                sea_orm::Value::Json(Some(Box::new(serde_json::to_value(model).unwrap())))
            }
        }
        
        impl ValueType for $field {
            fn try_from(value: Value) -> Result<Self, ValueTypeErr> {
                match value {
                    Value::Json(Some(x)) => serde_json::from_value::<$field>(*x).map_err(|_| ValueTypeErr),
                    _ => Err(ValueTypeErr)
                }
            }
        
            fn type_name() -> String {
                String::from(stringify!($field))
            }
        
            fn column_type() -> sea_orm::sea_query::ColumnType {
                sea_orm::sea_query::ColumnType::Json
            }
        }
        
        impl TryGetable for $field {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, sea_orm::TryGetError> {
                let value: Json = res.try_get(pre, col).map_err(sea_orm::TryGetError::DbErr)?;
                Ok(serde_json::from_value::<$field>(value).map_err(|_| sea_orm::TryGetError::DbErr(DbErr::Json(String::from("Failed to parse"))))?)
            }
        }
    };
}
