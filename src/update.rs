use crate::where_clause::WhereClauses;
use crate::SqlBuilder;

pub struct UpdateQuery {
    pub table: &'static str,
    pub columns: Vec<&'static str>,
    pub where_clause: WhereClauses,
}

impl UpdateQuery {
    pub fn new(table: &'static str) -> Self {
        UpdateQuery {
            table,
            columns: vec![],
            where_clause: WhereClauses::new(),
        }
    }

    pub fn add_column(&mut self, column: &'static str) {
        self.columns.push(column);
    }

    pub fn add_columns(&mut self, columns: Vec<&'static str>) {
        self.columns.extend(columns);
    }
}

impl SqlBuilder for UpdateQuery {
    fn build(&self) -> String {
        let mut sql = String::from("UPDATE ");
        sql.push_str(self.table);
        sql.push_str(" SET ");
        sql.push_str(
            &self
                .columns
                .iter()
                .map(|c| format!("{} = ?", c))
                .collect::<Vec<String>>()
                .join(", "),
        );
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
    fn test_update() {
        use super::*;
        let mut update = UpdateQuery::new("users");
        update.add_column("name");
        update.add_column("email");
        assert_eq!(update.build(), "UPDATE users SET name = ?, email = ?");
        update.where_clause.and_eq("id");
        assert_eq!(
            update.build(),
            "UPDATE users SET name = ?, email = ? WHERE id = ?"
        );
    }
}
