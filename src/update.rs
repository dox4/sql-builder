use crate::Result;
use crate::{repr::ToSqlRepr, where_clause::WhereClause, SqlBuilder};

#[derive(Debug, Clone)]
pub struct UpdateQuery {
    pub table: &'static str,
    pub fields: Vec<(&'static str, String)>,
    pub where_clause: Option<WhereClause>,
}

impl UpdateQuery {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            fields: Vec::new(),
            where_clause: None,
        }
    }

    pub fn set_field<T: ToSqlRepr>(&mut self, field: &'static str, value: &T) -> &mut Self {
        self.fields.push((field, value.to_sql_repr()));
        self
    }

    pub fn add_where_clause(&mut self, where_clause: WhereClause) -> &mut Self {
        self.where_clause = Some(where_clause);
        self
    }
}

impl SqlBuilder for UpdateQuery {
    fn build(&self) -> Result<String> {
        if self.fields.is_empty() {
            return Err(crate::error::Error::NoUpdateFields);
        }

        let fields = self
            .fields
            .iter()
            .map(|(field, value)| format!("{} = {}", field, value))
            .collect::<Vec<String>>()
            .join(", ");
        if let Some(where_clause) = &self.where_clause {
            Ok(format!(
                "UPDATE {} SET {} WHERE {}",
                self.table,
                fields,
                where_clause.build()?
            ))
        } else {
            Err(crate::error::Error::NoUpdateConditions)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::where_clause::WhereClause;

    #[test]
    fn test_update_query() {
        let mut update_query = UpdateQuery::new("users");
        update_query
            .set_field("name", &"John")
            .set_field("age", &30)
            .add_where_clause(WhereClause::equals("id", 1));
        assert_eq!(
            update_query.build().unwrap(),
            "UPDATE users SET name = 'John', age = 30 WHERE id = 1"
        );
    }

    #[test]
    fn test_update_query_no_fields() {
        let update_query = UpdateQuery::new("users");
        assert_eq!(
            update_query.build().unwrap_err(),
            crate::error::Error::NoUpdateFields
        );
    }

    #[test]
    fn test_update_query_no_conditions() {
        let mut update_query = UpdateQuery::new("users");
        update_query.set_field("name", &"John");
        assert_eq!(
            update_query.build().unwrap_err(),
            crate::error::Error::NoUpdateConditions
        );
    }
}
