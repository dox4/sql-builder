use crate::SqlBuilder;

pub struct Insert {
    pub table: &'static str,
    pub columns: Vec<&'static str>,
}

impl Insert {
    pub fn new(table: &'static str) -> Self {
        Insert {
            table,
            columns: vec![],
        }
    }

    pub fn add_column(&mut self, column: &'static str) {
        self.columns.push(column);
    }

    pub fn add_columns(&mut self, columns: Vec<&'static str>) {
        self.columns.extend(columns);
    }

    pub fn build_bulk(&self, count: usize) -> String {
        let mut sql = String::from("INSERT INTO ");
        sql.push_str(self.table);
        sql.push_str(" (");
        sql.push_str(&self.columns.join(", "));
        sql.push_str(") VALUES ");
        sql.push_str(
            &std::iter::repeat(format!(
                "({})",
                self.columns
                    .iter()
                    .map(|_| "?")
                    .collect::<Vec<&str>>()
                    .join(", ")
            ))
            .take(count)
            .collect::<Vec<String>>()
            .join(", "),
        );
        sql
    }
}

impl SqlBuilder for Insert {
    fn build(&self) -> String {
        let mut sql = String::from("INSERT INTO ");
        sql.push_str(self.table);
        sql.push_str(" (");
        sql.push_str(&self.columns.join(", "));
        sql.push_str(") VALUES (");
        sql.push_str(
            &self
                .columns
                .iter()
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(", "),
        );
        sql.push_str(")");
        sql
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        let mut insert = Insert::new("users");
        insert.add_column("name");
        insert.add_column("email");
        insert.add_column("password");
        assert_eq!(
            insert.build(),
            "INSERT INTO users (name, email, password) VALUES (?, ?, ?)"
        );
    }

    #[test]
    fn test_build_bulk() {
        let mut insert = Insert::new("users");
        insert.add_column("name");
        insert.add_column("email");
        insert.add_column("password");
        assert_eq!(
            insert.build_bulk(2),
            "INSERT INTO users (name, email, password) VALUES (?, ?, ?), (?, ?, ?)"
        );
    }
}
