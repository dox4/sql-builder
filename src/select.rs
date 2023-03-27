use crate::where_clause::WhereClause;
use crate::SqlBuilder;

#[derive(Debug, Clone, Default)]
pub struct SelectQuery {
    table: &'static str,
    columns: Vec<&'static str>,
    where_clause: Option<WhereClause>,
    order_by: Vec<OrderBy>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SelectQuery {
    pub fn new(table: &'static str) -> Self {
        SelectQuery {
            table,
            columns: vec![],
            ..Default::default()
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

    pub fn order_by(&mut self, field: &'static str) -> &mut Self {
        self.order_by.push(OrderBy::order_by(field));
        self
    }

    pub fn order_by_columns(&mut self, columns: Vec<&'static str>) -> &mut Self {
        self.order_by.push(OrderBy::order_by_columns(columns));
        self
    }

    pub fn desc(&mut self) -> &mut Self {
        self.order_by.last_mut().unwrap().desc();
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.order_by.last_mut().unwrap().asc();
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

impl SqlBuilder for SelectQuery {
    fn build(&self) -> String {
        let mut sql = format!(
            "SELECT {} FROM {}",
            if self.columns.is_empty() {
                "*".to_string()
            } else {
                self.columns.join(", ")
            },
            self.table
        );

        if self.where_clause.is_some() {
            sql.push_str(&format!(
                " WHERE {}",
                &self.where_clause.as_ref().unwrap().build()
            ));
        }

        if !self.order_by.is_empty() {
            sql.push_str(&format!(
                " ORDER BY {}",
                self.order_by
                    .iter()
                    .map(|order_by| order_by.build())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub columns: Vec<&'static str>,
    pub order: Order,
}

impl OrderBy {
    pub fn order_by(field: &'static str) -> Self {
        OrderBy {
            columns: vec![field],
            order: Order::Asc,
        }
    }

    pub fn order_by_columns(columns: Vec<&'static str>) -> Self {
        OrderBy {
            columns,
            order: Order::Asc,
        }
    }

    pub fn desc(&mut self) {
        self.order = Order::Desc;
    }

    pub fn asc(&mut self) {
        self.order = Order::Asc;
    }
}

impl SqlBuilder for OrderBy {
    fn build(&self) -> String {
        format!("{} {}", self.columns.join(", "), self.order.build())
    }
}

#[derive(Debug, Clone, Default)]
pub enum Order {
    #[default]
    Asc,
    Desc,
}

impl SqlBuilder for Order {
    fn build(&self) -> String {
        match self {
            Order::Asc => "ASC".to_string(),
            Order::Desc => "DESC".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_order_by() {
        use super::*;

        let mut order_by = OrderBy {
            columns: vec!["id", "name"],
            order: Order::Asc,
        };

        assert_eq!(order_by.build(), "id, name ASC");

        order_by.desc();
        assert_eq!(order_by.build(), "id, name DESC");
    }

    #[test]
    fn test_select_query() {
        use super::*;

        let mut select_query = SelectQuery::new("users");
        assert_eq!(select_query.build(), "SELECT * FROM users");
        select_query.add_column("id").add_column("name");
        assert_eq!(select_query.build(), "SELECT id, name FROM users");
        let where_clause = WhereClause::equal("id");
        select_query.where_clause(where_clause);
        assert_eq!(
            select_query.build(),
            "SELECT id, name FROM users WHERE id = ?"
        );
        select_query
            .order_by_columns(vec!["id", "name"])
            .asc()
            .limit(10)
            .offset(20);

        assert_eq!(
            select_query.build(),
            "SELECT id, name FROM users WHERE id = ? ORDER BY id, name ASC LIMIT 10 OFFSET 20"
        );
    }
}
