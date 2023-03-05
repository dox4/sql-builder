use crate::where_clause::WhereClauses;
use crate::SqlBuilder;

#[derive(Debug, Clone, Default)]
pub struct SelectQuery {
    table: &'static str,
    columns: Vec<&'static str>,
    where_clause: WhereClauses,
    order_by: Option<OrderBy>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SelectQuery {
    pub fn new(table: &'static str) -> Self {
        SelectQuery {
            table,
            columns: vec![],
            where_clause: WhereClauses::new(),
            ..Default::default()
        }
    }

    pub fn add_column(&mut self, column: &'static str) {
        self.columns.push(column);
    }

    pub fn add_columns(&mut self, columns: Vec<&'static str>) {
        self.columns.extend(columns);
    }

    pub fn add_where_clause(&mut self, where_clause: WhereClauses) {
        self.where_clause = where_clause;
    }

    pub fn add_order_by(&mut self, order_by: OrderBy) {
        self.order_by = Some(order_by);
    }

    pub fn add_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn add_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
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

        if !self.where_clause.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_clause.build()));
        }

        if let Some(order_by) = &self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order_by.build()));
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
        select_query.add_column("id");
        select_query.add_column("name");
        assert_eq!(select_query.build(), "SELECT id, name FROM users");
        let mut where_clause = WhereClauses::new();
        where_clause.and_eq("id");
        select_query.add_where_clause(where_clause);
        assert_eq!(
            select_query.build(),
            "SELECT id, name FROM users WHERE id = ?"
        );
        select_query.add_order_by(OrderBy {
            columns: vec!["id", "name"],
            order: Order::Asc,
        });
        select_query.add_limit(10);
        select_query.add_offset(20);

        assert_eq!(
            select_query.build(),
            "SELECT id, name FROM users WHERE id = ? ORDER BY id, name ASC LIMIT 10 OFFSET 20"
        );
    }
}
