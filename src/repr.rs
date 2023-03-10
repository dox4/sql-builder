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

