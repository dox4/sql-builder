pub trait ToSqlRepr {
    fn to_sql_repr(&self) -> String;
}

macro_rules! impl_to_sql_repr {
    ($($t:ty),*) => {
        $(
            impl ToSqlRepr for $t {
                fn to_sql_repr(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    }
}

impl_to_sql_repr!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

impl ToSqlRepr for bool {
    fn to_sql_repr(&self) -> String {
        if *self {
            "TRUE".to_string()
        } else {
            "FALSE".to_string()
        }
    }
}

impl ToSqlRepr for String {
    fn to_sql_repr(&self) -> String {
        format!("'{}'", self)
    }
}

impl ToSqlRepr for &str {
    fn to_sql_repr(&self) -> String {
        format!("'{}'", self)
    }
}

impl<T> ToSqlRepr for Option<T>
where
    T: ToSqlRepr,
{
    fn to_sql_repr(&self) -> String {
        match self {
            Some(v) => v.to_sql_repr(),
            None => "NULL".to_string(),
        }
    }
}

use chrono::{DateTime, Local};
impl ToSqlRepr for DateTime<Local> {
    fn to_sql_repr(&self) -> String {
        format!("'{}'", self.format("%Y-%m-%d %H:%M:%S"))
    }
}

use serde_json::Value;
impl ToSqlRepr for Value {
    fn to_sql_repr(&self) -> String {
        format!("'{}'", self.to_string())
    }
}

use uuid::Uuid;
impl ToSqlRepr for Uuid {
    fn to_sql_repr(&self) -> String {
        format!("UUID_TO_BIN('{}')", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_sql_repr() {
        assert_eq!(1.to_sql_repr(), "1");
        assert_eq!(1.1.to_sql_repr(), "1.1");
        assert_eq!(true.to_sql_repr(), "TRUE");
        assert_eq!(false.to_sql_repr(), "FALSE");
        assert_eq!("hello".to_sql_repr(), "'hello'");
        assert_eq!(Some(1).to_sql_repr(), "1");
        assert_eq!(Some("hello").to_sql_repr(), "'hello'");
        assert_eq!(Some(true).to_sql_repr(), "TRUE");
        assert_eq!(Some(false).to_sql_repr(), "FALSE");
        assert_eq!(None::<i32>.to_sql_repr(), "NULL");
        assert_eq!(None::<String>.to_sql_repr(), "NULL");
        assert_eq!(None::<bool>.to_sql_repr(), "NULL");
    }

    #[test]
    fn test_datetime() {
        let now = Local::now();
        assert_eq!(
            now.to_sql_repr(),
            format!("'{}'", now.format("%Y-%m-%d %H:%M:%S"))
        );
    }

    #[test]
    fn test_json() {
        let json = serde_json::json!({"hello": "world"});
        assert_eq!(json.to_sql_repr(), "'{\"hello\":\"world\"}'");
    }

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4();
        assert_eq!(
            uuid.to_sql_repr(),
            format!("UUID_TO_BIN('{}')", uuid.to_string())
        );
    }
}
