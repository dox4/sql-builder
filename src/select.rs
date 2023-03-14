use crate::error::Error;
use crate::Result;
use crate::{where_clause::WhereClause, SqlBuilder};

#[derive(Debug, Clone)]
pub struct SelectQuery {
    table: &'static str,
    columns: Vec<&'static str>,
    where_clause: Option<WhereClause>,
    limit: Option<u64>,
    offset: Option<u64>,
    order_by: Vec<OrderBy>,
}

impl SelectQuery {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            columns: Vec::new(),
            where_clause: None,
            limit: None,
            offset: None,
            order_by: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: &'static str) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn add_columns(&mut self, columns: &[&'static str]) -> &mut Self {
        self.columns.extend(columns);
        self
    }

    pub fn where_clause(&mut self, where_clause: WhereClause) -> &mut Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(&mut self, column: &'static str) -> &mut Self {
        self.order_by.push(OrderBy::new(column));
        self
    }

    pub fn order_by_columns(&mut self, columns: &[&'static str]) -> &mut Self {
        self.order_by.push(OrderBy::from_columns(columns));
        self
    }

    pub fn desc(&mut self) -> Result<&mut Self> {
        if self.order_by.is_empty() {
            return Err(Error::NoOrderByClause);
        }
        self.order_by.last_mut().unwrap().desc();
        Ok(self)
    }

    pub fn asc(&mut self) -> Result<&mut Self> {
        if self.order_by.is_empty() {
            return Err(Error::NoOrderByClause);
        }
        self.order_by.last_mut().unwrap().asc();
        Ok(self)
    }

    pub fn count(&mut self) -> &mut Self {
        self.columns.clear();
        self.columns.push("COUNT(*)");
        self
    }
}

impl SqlBuilder for SelectQuery {
    fn build(&self) -> Result<String> {
        let mut sql = String::new();
        sql.push_str("SELECT ");
        if self.columns.is_empty() {
            sql.push_str("*");
        } else {
            sql.push_str(&self.columns.join(", "));
        }
        sql.push_str(" FROM ");
        sql.push_str(self.table);
        if let Some(where_clause) = &self.where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause.build()?);
        }
        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(
                &self
                    .order_by
                    .iter()
                    .map(|o| o.build())
                    .collect::<Result<Vec<String>>>()?
                    .join(", "),
            );
        }
        if let Some(limit) = self.limit {
            sql.push_str(" LIMIT ");
            sql.push_str(&limit.to_string());
        }
        if let Some(offset) = self.offset {
            sql.push_str(" OFFSET ");
            sql.push_str(&offset.to_string());
        }
        Ok(sql)
    }
}

#[derive(Debug, Clone)]
enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
struct OrderBy {
    columns: Vec<&'static str>,
    order: Order,
}

impl OrderBy {
    pub fn new(column: &'static str) -> Self {
        Self {
            columns: vec![column],
            order: Order::Asc,
        }
    }
    pub fn from_columns(columns: &[&'static str]) -> Self {
        Self {
            columns: columns.to_vec(),
            order: Order::Asc,
        }
    }

    pub fn asc(&mut self) -> &mut Self {
        self.order = Order::Asc;
        self
    }

    pub fn desc(&mut self) -> &mut Self {
        self.order = Order::Desc;
        self
    }
}

impl SqlBuilder for OrderBy {
    fn build(&self) -> Result<String> {
        Ok(format!(
            "{} {}",
            self.columns.join(", "),
            match self.order {
                Order::Asc => "ASC",
                Order::Desc => "DESC",
            }
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::where_clause::WhereClause;

    #[test]
    fn test_basic_select() {
        let mut select = SelectQuery::new("users");
        select.add_column("id");
        let sql = select.build().unwrap();
        assert_eq!(sql, "SELECT id FROM users");
    }

    #[test]
    fn test_select_all() {
        let select = SelectQuery::new("users");
        let sql = select.build().unwrap();
        assert_eq!(sql, "SELECT * FROM users");
    }

    #[test]
    fn test_select() {
        let mut select = SelectQuery::new("users");
        select
            .add_column("id")
            .add_column("name")
            .add_column("email")
            .where_clause(WhereClause::equals("id", 1))
            .order_by("id")
            .desc()
            .unwrap()
            .limit(10)
            .offset(5);
        let sql = select.build().unwrap();
        assert_eq!(
            sql,
            "SELECT id, name, email FROM users WHERE id = 1 ORDER BY id DESC LIMIT 10 OFFSET 5"
        );
    }

    #[test]
    fn test_select_no_order_by() {
        let mut select = SelectQuery::new("users");
        select
            .add_column("id")
            .add_column("name")
            .add_column("email")
            .where_clause(WhereClause::equals("id", 1))
            .limit(10)
            .offset(5);
        let sql = select.build().unwrap();
        assert_eq!(
            sql,
            "SELECT id, name, email FROM users WHERE id = 1 LIMIT 10 OFFSET 5"
        );
    }

    #[test]
    fn test_select_no_where_clause() {
        let mut select = SelectQuery::new("users");
        select
            .add_column("id")
            .add_column("name")
            .add_column("email")
            .order_by("id")
            .desc()
            .unwrap()
            .limit(10)
            .offset(5);
        let sql = select.build().unwrap();
        assert_eq!(
            sql,
            "SELECT id, name, email FROM users ORDER BY id DESC LIMIT 10 OFFSET 5"
        );
    }

    #[test]
    fn test_select_no_limit() {
        let mut select = SelectQuery::new("users");
        select
            .add_column("id")
            .add_column("name")
            .add_column("email")
            .where_clause(WhereClause::equals("id", 1))
            .order_by("id")
            .desc()
            .unwrap()
            .offset(5);
        let sql = select.build().unwrap();
        assert_eq!(
            sql,
            "SELECT id, name, email FROM users WHERE id = 1 ORDER BY id DESC OFFSET 5"
        );
    }
}
