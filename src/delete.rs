use crate::where_clause::WhereClauses;
use crate::SqlBuilder;

pub struct DeleteQuery {
    pub table: &'static str,
    pub where_clause: WhereClauses,
}

impl DeleteQuery {
    pub fn new(table: &'static str) -> Self {
        DeleteQuery {
            table,
            where_clause: WhereClauses::new(),
        }
    }
}

impl SqlBuilder for DeleteQuery {
    fn build(&self) -> String {
        let mut sql = String::from("DELETE FROM ");
        sql.push_str(self.table);
        if self.where_clause.is_empty() {
            return sql;
        }
        sql.push_str(" WHERE ");
        sql.push_str(&self.where_clause.build());
        sql
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_delete() {
        use crate::SqlBuilder;
        use crate::delete::DeleteQuery;

        let mut delete = DeleteQuery::new("users");
        delete.where_clause.and_eq("id");
        assert_eq!(delete.build(), "DELETE FROM users WHERE id = ?");
    }
}
