use crate::where_clause::WhereClause;
use crate::SqlBuilder;

pub struct DeleteQuery {
    pub table: &'static str,
    pub where_clause: Option<WhereClause>,
}

impl DeleteQuery {
    pub fn new(table: &'static str) -> Self {
        DeleteQuery {
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
    fn build(&self) -> String {
        match self.where_clause {
            Some(ref where_clause) => {
                format!("DELETE FROM {} WHERE {}", self.table, where_clause.build())
            }
            None => {
                panic!("Delete query must have a where clause")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::where_clause::WhereClause;

    #[test]
    fn test_delete() {
        use crate::delete::DeleteQuery;
        use crate::SqlBuilder;

        let delete = DeleteQuery::new("users").where_clause(WhereClause::equal("id"));
        assert_eq!(delete.build(), "DELETE FROM users WHERE id = ?");
    }
}
