use crate::error::Error;
use crate::where_clause::WhereClause;
use crate::Result;
use crate::SqlBuilder;

#[derive(Debug, Clone)]
pub struct DeleteQuery {
    pub table: &'static str,
    pub where_clause: Option<WhereClause>,
}

impl DeleteQuery {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            where_clause: None,
        }
    }

    pub fn where_clause(mut self, where_clause: WhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
}

impl SqlBuilder for DeleteQuery {
    fn build(self: &DeleteQuery) -> Result<String> {
        if let Some(where_clause) = &self.where_clause {
            Ok(format!(
                "DELETE FROM {} WHERE {}",
                self.table,
                where_clause.build()?
            ))
        } else {
            Err(Error::NoDeleteConditions)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::where_clause::WhereClause;

    #[test]
    fn test_delete_without_where() {
        let delete = DeleteQuery::new("users").build();
        assert_eq!(delete, Err(Error::NoDeleteConditions));
    }

    #[test]
    fn test_delete() {
        let delete = DeleteQuery::new("users")
            .where_clause(WhereClause::equals("id", 1))
            .build();
        assert_eq!(delete, Ok("DELETE FROM users WHERE id = 1".to_string()));
    }
}
