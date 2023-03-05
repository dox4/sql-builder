use crate::SqlBuilder;

#[derive(Debug, Clone)]
pub enum WhereClause {
    And(Condition),
    Or(Condition),
    AndGroup(WhereClauses),
    OrGroup(WhereClauses),
}

#[derive(Debug, Clone, Default)]
pub struct WhereClauses {
    conditions: Vec<WhereClause>,
}

impl WhereClauses {
    pub fn new() -> Self {
        WhereClauses { conditions: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }
}

impl SqlBuilder for WhereClauses {
    fn build(&self) -> String {
        self.conditions
            .iter()
            .enumerate()
            .map(|(idx, cond)| {
                if idx == 0 {
                    match cond {
                        WhereClause::And(cond) | WhereClause::Or(cond) => cond.build(),
                        WhereClause::AndGroup(cond) | WhereClause::OrGroup(cond) => cond.build(),
                    }
                } else {
                    match cond {
                        WhereClause::And(cond) => format!("AND {}", cond.build()),
                        WhereClause::Or(cond) => format!("OR {}", cond.build()),
                        WhereClause::AndGroup(cond) => format!("AND ({})", cond.build()),
                        WhereClause::OrGroup(cond) => format!("OR ({})", cond.build()),
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl WhereClauses {
    pub fn and(&mut self, cond: Condition) {
        self.conditions.push(WhereClause::And(cond));
    }

    pub fn or(&mut self, cond: Condition) {
        self.conditions.push(WhereClause::Or(cond));
    }

    pub fn and_where(&mut self, cond: WhereClauses) {
        self.conditions.push(WhereClause::AndGroup(cond));
    }

    pub fn or_where(&mut self, cond: WhereClauses) {
        self.conditions.push(WhereClause::OrGroup(cond));
    }

    pub fn and_eq(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::Equal,
        });
    }

    pub fn and_not_eq(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::NotEqual,
        });
    }

    pub fn and_gt(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::GreaterThan,
        });
    }

    pub fn and_gte(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::GreaterThanOrEqual,
        });
    }

    pub fn and_lt(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::LessThan,
        });
    }

    pub fn and_lte(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::LessThanOrEqual,
        });
    }

    pub fn and_between(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::Between,
        });
    }

    pub fn and_in(&mut self, column: &'static str, cnt: usize) {
        self.and(Condition {
            column,
            operator: Operator::In(cnt),
        });
    }

    pub fn and_like(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::Like,
        });
    }

    pub fn and_is_null(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::IsNull,
        });
    }

    pub fn and_is_not_null(&mut self, column: &'static str) {
        self.and(Condition {
            column,
            operator: Operator::IsNotNull,
        });
    }

    pub fn or_eq(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::Equal,
        });
    }

    pub fn or_not_eq(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::NotEqual,
        });
    }

    pub fn or_gt(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::GreaterThan,
        });
    }

    pub fn or_gte(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::GreaterThanOrEqual,
        });
    }

    pub fn or_lt(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::LessThan,
        });
    }

    pub fn or_lte(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::LessThanOrEqual,
        });
    }

    pub fn or_between(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::Between,
        });
    }

    pub fn or_in(&mut self, column: &'static str, cnt: usize) {
        self.or(Condition {
            column,
            operator: Operator::In(cnt),
        });
    }

    pub fn or_like(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::Like,
        });
    }

    pub fn or_is_null(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::IsNull,
        });
    }

    pub fn or_is_not_null(&mut self, column: &'static str) {
        self.or(Condition {
            column,
            operator: Operator::IsNotNull,
        });
    }
}

#[derive(Debug, Clone, Default)]
pub struct Condition {
    pub column: &'static str,
    pub operator: Operator,
}

impl SqlBuilder for Condition {
    fn build(&self) -> String {
        match self.operator {
            Operator::Equal => format!("{} = ?", self.column),
            Operator::NotEqual => format!("{} != ?", self.column),
            Operator::GreaterThan => format!("{} > ?", self.column),
            Operator::GreaterThanOrEqual => format!("{} >= ?", self.column),
            Operator::LessThan => format!("{} < ?", self.column),
            Operator::LessThanOrEqual => format!("{} <= ?", self.column),
            Operator::Between => format!("{} BETWEEN ? AND ?", self.column),
            Operator::In(cnt) => format!("{} IN ({})", self.column, vec!["?"; cnt].join(", ")),
            Operator::Like => format!("{} LIKE ?", self.column),
            Operator::IsNull => format!("{} IS NULL", self.column),
            Operator::IsNotNull => format!("{} IS NOT NULL", self.column),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Operator {
    #[default]
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Between,
    In(usize),
    Like,
    IsNull,
    IsNotNull,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_where_clause_and() {
        use super::*;

        let mut where_clauses = WhereClauses::new();
        where_clauses.and_eq("id");
        where_clauses.and_not_eq("name");
        where_clauses.and_gt("age");
        where_clauses.and_gte("height");
        where_clauses.and_lt("weight");
        where_clauses.and_lte("score");
        where_clauses.and_between("money");
        where_clauses.and_in("hobby", 3);
        where_clauses.and_like("address");
        where_clauses.and_is_null("created_at");
        where_clauses.and_is_not_null("updated_at");
        assert_eq!(
            where_clauses.build(),
            "id = ? AND name != ? AND age > ? AND height >= ? AND weight < ? AND score <= ? AND money BETWEEN ? AND ? AND hobby IN (?, ?, ?) AND address LIKE ? AND created_at IS NULL AND updated_at IS NOT NULL"
        );
    }

    #[test]
    fn test_where_clause_or() {
        use super::*;

        let mut where_clauses = WhereClauses::new();
        where_clauses.or_eq("id");
        where_clauses.or_not_eq("name");
        where_clauses.or_gt("age");
        where_clauses.or_gte("height");
        where_clauses.or_lt("weight");
        where_clauses.or_lte("score");
        where_clauses.or_between("money");
        where_clauses.or_in("hobby", 3);
        where_clauses.or_like("address");
        where_clauses.or_is_null("created_at");
        where_clauses.or_is_not_null("updated_at");
        assert_eq!(
            where_clauses.build(),
            "id = ? OR name != ? OR age > ? OR height >= ? OR weight < ? OR score <= ? OR money BETWEEN ? AND ? OR hobby IN (?, ?, ?) OR address LIKE ? OR created_at IS NULL OR updated_at IS NOT NULL"
        );
    }

    #[test]
    fn test_where_clause_and_where() {
        use super::*;

        let mut where_clauses1 = WhereClauses::new();
        where_clauses1.and_eq("id");
        where_clauses1.and_not_eq("name");
        let mut where_clauses2 = WhereClauses::new();
        where_clauses2.and_gt("age");
        where_clauses2.and_gte("height");
        where_clauses1.and_where(where_clauses2);
        assert_eq!(
            where_clauses1.build(),
            "id = ? AND name != ? AND (age > ? AND height >= ?)"
        );
    }

    #[test]
    fn test_where_clause_or_where() {
        use super::*;

        let mut where_clauses1 = WhereClauses::new();
        where_clauses1.or_eq("id");
        where_clauses1.or_not_eq("name");
        let mut where_clauses2 = WhereClauses::new();
        where_clauses2.or_gt("age");
        where_clauses2.or_gte("height");
        where_clauses1.or_where(where_clauses2);
        assert_eq!(
            where_clauses1.build(),
            "id = ? OR name != ? OR (age > ? OR height >= ?)"
        );
    }
}
