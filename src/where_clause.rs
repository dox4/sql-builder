use std::iter::repeat;

use crate::SqlBuilder;

#[derive(Debug, Clone)]
pub enum WhereCond {
    Equal(&'static str),
    NotEqual(&'static str),
    GreaterThan(&'static str),
    GreaterThanOrEqual(&'static str),
    LessThan(&'static str),
    LessThanOrEqual(&'static str),
    Between(&'static str),
    NotBetween(&'static str),
    In(&'static str, usize),
    NotIn(&'static str, usize),
    Like(&'static str),
    NotLike(&'static str),
    IsNull(&'static str),
    IsNotNull(&'static str),
    RawCondition(&'static str),
}

impl WhereCond {
    fn equal(field: &'static str) -> Self {
        WhereCond::Equal(field)
    }

    fn not_equal(field: &'static str) -> Self {
        WhereCond::NotEqual(field)
    }

    fn greater_than(field: &'static str) -> Self {
        WhereCond::GreaterThan(field)
    }

    fn greater_than_or_equal(field: &'static str) -> Self {
        WhereCond::GreaterThanOrEqual(field)
    }

    fn less_than(field: &'static str) -> Self {
        WhereCond::LessThan(field)
    }

    fn less_than_or_equal(field: &'static str) -> Self {
        WhereCond::LessThanOrEqual(field)
    }

    fn between(field: &'static str) -> Self {
        WhereCond::Between(field)
    }

    fn not_between(field: &'static str) -> Self {
        WhereCond::NotBetween(field)
    }

    fn in_(field: &'static str, count: usize) -> Self {
        WhereCond::In(field, count)
    }

    fn not_in(field: &'static str, count: usize) -> Self {
        WhereCond::NotIn(field, count)
    }

    fn like(field: &'static str) -> Self {
        WhereCond::Like(field)
    }

    fn not_like(field: &'static str) -> Self {
        WhereCond::NotLike(field)
    }

    fn is_null(field: &'static str) -> Self {
        WhereCond::IsNull(field)
    }

    fn is_not_null(field: &'static str) -> Self {
        WhereCond::IsNotNull(field)
    }

    fn raw_condition(condition: &'static str) -> Self {
        WhereCond::RawCondition(condition)
    }
}

#[derive(Debug, Clone)]
pub enum WhereClause {
    And(Box<WhereClause>, Box<WhereClause>),
    Or(Box<WhereClause>, Box<WhereClause>),
    Condition(WhereCond),
}

impl WhereClause {
    pub fn and(&mut self, other: WhereClause) -> Self {
        WhereClause::And(Box::new(self.clone()), Box::new(other))
    }

    pub fn or(&mut self, other: WhereClause) -> Self {
        WhereClause::Or(Box::new(self.clone()), Box::new(other))
    }

    pub fn condition(cond: WhereCond) -> Self {
        WhereClause::Condition(cond)
    }

    pub fn equal(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::equal(field))
    }

    pub fn not_equal(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::not_equal(field))
    }

    pub fn greater_than(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::greater_than(field))
    }

    pub fn greater_than_or_equal(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::greater_than_or_equal(field))
    }

    pub fn less_than(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::less_than(field))
    }

    pub fn less_than_or_equal(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::less_than_or_equal(field))
    }

    pub fn between(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::between(field))
    }

    pub fn not_between(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::not_between(field))
    }

    pub fn in_(field: &'static str, count: usize) -> Self {
        WhereClause::condition(WhereCond::in_(field, count))
    }

    pub fn not_in(field: &'static str, count: usize) -> Self {
        WhereClause::condition(WhereCond::not_in(field, count))
    }

    pub fn like(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::like(field))
    }

    pub fn not_like(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::not_like(field))
    }

    pub fn is_null(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::is_null(field))
    }

    pub fn is_not_null(field: &'static str) -> Self {
        WhereClause::condition(WhereCond::is_not_null(field))
    }

    pub fn raw_condition(condition: &'static str) -> Self {
        WhereClause::condition(WhereCond::raw_condition(condition))
    }

    pub fn and_equal(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::equal(field)))
    }

    pub fn and_not_equal(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::not_equal(field)))
    }

    pub fn and_greater_than(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::greater_than(field)))
    }

    pub fn and_greater_than_or_equal(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::greater_than_or_equal(
            field,
        )))
    }

    pub fn and_less_than(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::less_than(field)))
    }

    pub fn and_less_than_or_equal(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::less_than_or_equal(field)))
    }

    pub fn and_between(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::between(field)))
    }

    pub fn and_not_between(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::not_between(field)))
    }

    pub fn and_in(&mut self, field: &'static str, count: usize) -> Self {
        self.and(WhereClause::condition(WhereCond::in_(field, count)))
    }

    pub fn and_not_in(&mut self, field: &'static str, count: usize) -> Self {
        self.and(WhereClause::condition(WhereCond::not_in(field, count)))
    }

    pub fn and_like(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::like(field)))
    }

    pub fn and_not_like(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::not_like(field)))
    }

    pub fn and_is_null(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::is_null(field)))
    }

    pub fn and_is_not_null(&mut self, field: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::is_not_null(field)))
    }

    pub fn and_raw_condition(&mut self, condition: &'static str) -> Self {
        self.and(WhereClause::condition(WhereCond::raw_condition(condition)))
    }

    pub fn or_equal(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::equal(field)))
    }

    pub fn or_not_equal(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::not_equal(field)))
    }

    pub fn or_greater_than(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::greater_than(field)))
    }

    pub fn or_greater_than_or_equal(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::greater_than_or_equal(
            field,
        )))
    }

    pub fn or_less_than(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::less_than(field)))
    }

    pub fn or_less_than_or_equal(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::less_than_or_equal(field)))
    }

    pub fn or_between(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::between(field)))
    }

    pub fn or_not_between(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::not_between(field)))
    }

    pub fn or_in(&mut self, field: &'static str, count: usize) -> Self {
        self.or(WhereClause::condition(WhereCond::in_(field, count)))
    }

    pub fn or_not_in(&mut self, field: &'static str, count: usize) -> Self {
        self.or(WhereClause::condition(WhereCond::not_in(field, count)))
    }

    pub fn or_like(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::like(field)))
    }

    pub fn or_not_like(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::not_like(field)))
    }

    pub fn or_is_null(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::is_null(field)))
    }

    pub fn or_is_not_null(&mut self, field: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::is_not_null(field)))
    }

    pub fn or_raw_condition(&mut self, condition: &'static str) -> Self {
        self.or(WhereClause::condition(WhereCond::raw_condition(condition)))
    }
}

impl SqlBuilder for WhereCond {
    fn build(&self) -> String {
        match *self {
            Self::Equal(ref field) => format!("{} = ?", field),
            Self::NotEqual(ref field) => format!("{} != ?", field),
            Self::GreaterThan(ref field) => format!("{} > ?", field),
            Self::GreaterThanOrEqual(ref field) => format!("{} >= ?", field),
            Self::LessThan(ref field) => format!("{} < ?", field),
            Self::LessThanOrEqual(ref field) => format!("{} <= ?", field),
            Self::Between(ref field) => format!("{} BETWEEN ? AND ?", field),
            Self::NotBetween(ref field) => format!("{} NOT BETWEEN ? AND ?", field),
            Self::In(ref field, count) => format!(
                "{} IN ({})",
                field,
                repeat("?, ")
                    .take(count)
                    .collect::<String>()
                    .trim_end_matches(", ")
            ),
            Self::NotIn(ref field, count) => format!(
                "{} NOT IN ({})",
                field,
                repeat("?, ")
                    .take(count)
                    .collect::<String>()
                    .trim_end_matches(", ")
            ),
            Self::Like(ref field) => format!("{} LIKE ?", field),
            Self::NotLike(ref field) => format!("{} NOT LIKE ?", field),
            Self::IsNull(ref field) => format!("{} IS NULL", field),
            Self::IsNotNull(ref field) => format!("{} IS NOT NULL", field),
            Self::RawCondition(ref condition) => condition.to_string(),
        }
    }
}

impl SqlBuilder for WhereClause {
    fn build(&self) -> String {
        match *self {
            WhereClause::Condition(ref condition) => condition.build(),
            WhereClause::And(ref left, ref right) => {
                let left: String = match left.as_ref() {
                    WhereClause::Or(_, _) => {
                        format!("({})", left.build())
                    }
                    _ => left.build(),
                };
                let right: String = match right.as_ref() {
                    WhereClause::Or(_, _) => {
                        format!("({})", right.build())
                    }
                    _ => right.build(),
                };

                format!("{} AND {}", left, right)
            }
            WhereClause::Or(ref left, ref right) => {
                format!("{} OR {}", left.build(), right.build())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .and(WhereClause::condition(WhereCond::equal("name")))
            .or(WhereClause::condition(WhereCond::equal("email")));

        assert_eq!(where_clause.build(), "id = ? AND name = ? OR email = ?");
    }

    #[test]
    fn test_and() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .and_equal("name")
            .and_equal("email");

        assert_eq!(where_clause.build(), "id = ? AND name = ? AND email = ?");
    }

    #[test]
    fn test_or() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .or_equal("name")
            .or_equal("email");

        assert_eq!(where_clause.build(), "id = ? OR name = ? OR email = ?");
    }
    #[test]
    fn test_or_and() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .or_equal("name")
            .and_equal("email");
        assert_eq!(where_clause.build(), "(id = ? OR name = ?) AND email = ?");
    }

    #[test]
    fn test_and_or() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .and_equal("name")
            .or_equal("email");

        assert_eq!(where_clause.build(), "id = ? AND name = ? OR email = ?");
    }

    #[test]
    fn test_nest_or() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .or_equal("name")
            .or(WhereClause::condition(WhereCond::equal("email")).or_equal("phone"));

        assert_eq!(
            where_clause.build(),
            "id = ? OR name = ? OR email = ? OR phone = ?"
        );
    }

    #[test]
    fn test_nest_and() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .and_equal("name")
            .and(WhereClause::condition(WhereCond::equal("email")).and_equal("phone"));

        assert_eq!(
            where_clause.build(),
            "id = ? AND name = ? AND email = ? AND phone = ?"
        );
    }

    #[test]
    fn test_nest_and_in_or() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .and_equal("name")
            .and(WhereClause::condition(WhereCond::equal("email")).or_equal("phone"));

        assert_eq!(
            where_clause.build(),
            "id = ? AND name = ? AND (email = ? OR phone = ?)"
        );
    }

    #[test]
    fn test_nest_or_in_and() {
        let where_clause = WhereClause::condition(WhereCond::equal("id"))
            .or_equal("name")
            .or(WhereClause::condition(WhereCond::equal("email")).and_equal("phone"));

        assert_eq!(
            where_clause.build(),
            "id = ? OR name = ? OR email = ? AND phone = ?"
        );
    }
}
