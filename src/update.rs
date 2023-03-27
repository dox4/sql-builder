use crate::where_clause::WhereClause;
use crate::SqlBuilder;

pub struct UpdateQuery {
    pub table: &'static str,
    pub columns: Vec<&'static str>,
    pub where_clause: Option<WhereClause>,
}

impl UpdateQuery {
    pub fn new(table: &'static str) -> Self {
        UpdateQuery {
            table,
            columns: vec![],
            where_clause: None,
        }
    }

    pub fn add_column(&mut self, column: &'static str) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn add_columns(&mut self, columns: Vec<&'static str>) -> &mut Self {
        self.columns.extend(columns);
        self
    }

    pub fn where_clause(&mut self, where_clause: WhereClause) -> &mut Self {
        self.where_clause = Some(where_clause);
        self
    }
}

impl SqlBuilder for UpdateQuery {
    fn build(&self) -> String {
        if self.columns.is_empty() {
            panic!("No columns specified for update query");
        }
        let updated_columns = self
            .columns
            .iter()
            .map(|c| format!("{} = ?", c))
            .collect::<Vec<String>>()
            .join(", ");

        match self.where_clause {
            Some(ref where_clause) => {
                format!(
                    "UPDATE {} SET {} WHERE {}",
                    self.table,
                    updated_columns,
                    where_clause.build()
                )
            }
            None => format!("UPDATE {} SET {}", self.table, updated_columns),
        }
    }
}

#[cfg(test)]

mod test {
    #[test]
    fn test_update() {
        use super::*;
        let mut update = UpdateQuery::new("users");
        update.add_column("name").add_column("email");
        assert_eq!(update.build(), "UPDATE users SET name = ?, email = ?");
        update.where_clause(WhereClause::equal("id"));
        assert_eq!(
            update.build(),
            "UPDATE users SET name = ?, email = ? WHERE id = ?"
        );
    }
}
