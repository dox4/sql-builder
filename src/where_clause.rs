use crate::{repr::ToSqlRepr, Result, SqlBuilder};

#[derive(Debug, Clone)]
pub enum WhereCondition {
    Equals(&'static str, String),
    NotEquals(&'static str, String),
    GreaterThan(&'static str, String),
    GreaterThanOrEquals(&'static str, String),
    LessThan(&'static str, String),
    LessThanOrEquals(&'static str, String),
    Like(&'static str, String),
    NotLike(&'static str, String),
    In(&'static str, Vec<String>),
    NotIn(&'static str, Vec<String>),
    Between(&'static str, String, String),
    NotBetween(&'static str, String, String),
    IsNull(&'static str),
    IsNotNull(&'static str),
    RawCondition(String),
}

#[derive(Debug, Clone)]
pub enum WhereClause {
    And(Box<WhereClause>, Box<WhereClause>),
    Or(Box<WhereClause>, Box<WhereClause>),
    Condition(WhereCondition),
}

impl WhereClause {
    pub fn equals<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::Equals(field, value.to_sql_repr()))
    }

    pub fn not_equals<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::NotEquals(field, value.to_sql_repr()))
    }

    pub fn greater_than<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::GreaterThan(field, value.to_sql_repr()))
    }

    pub fn greater_than_or_equals<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::GreaterThanOrEquals(
            field,
            value.to_sql_repr(),
        ))
    }

    pub fn less_than<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::LessThan(field, value.to_sql_repr()))
    }

    pub fn less_than_or_equals<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::LessThanOrEquals(field, value.to_sql_repr()))
    }

    pub fn like<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::Like(field, value.to_sql_repr()))
    }

    pub fn not_like<T: ToSqlRepr>(field: &'static str, value: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::NotLike(field, value.to_sql_repr()))
    }

    pub fn in_<T: ToSqlRepr>(field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::Condition(WhereCondition::In(
            field,
            values.into_iter().map(|v| v.to_sql_repr()).collect(),
        ))
    }

    pub fn not_in<T: ToSqlRepr>(field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::Condition(WhereCondition::NotIn(
            field,
            values.into_iter().map(|v| v.to_sql_repr()).collect(),
        ))
    }

    pub fn between<T: ToSqlRepr>(field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::Between(
            field,
            min.to_sql_repr(),
            max.to_sql_repr(),
        ))
    }

    pub fn not_between<T: ToSqlRepr>(field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::Condition(WhereCondition::NotBetween(
            field,
            min.to_sql_repr(),
            max.to_sql_repr(),
        ))
    }

    pub fn is_null(field: &'static str) -> WhereClause {
        WhereClause::Condition(WhereCondition::IsNull(field))
    }

    pub fn is_not_null(field: &'static str) -> WhereClause {
        WhereClause::Condition(WhereCondition::IsNotNull(field))
    }

    pub fn raw_condition(condition: String) -> WhereClause {
        WhereClause::Condition(WhereCondition::RawCondition(condition))
    }

    pub fn and(self, other: WhereClause) -> WhereClause {
        WhereClause::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: WhereClause) -> WhereClause {
        WhereClause::Or(Box::new(self), Box::new(other))
    }

    pub fn and_equals<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Equals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_not_equals<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_greater_than<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::GreaterThan(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_greater_than_or_equals<T: ToSqlRepr>(
        self,
        field: &'static str,
        value: T,
    ) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::GreaterThanOrEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_less_than<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::LessThan(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_less_than_or_equals<T: ToSqlRepr>(
        self,
        field: &'static str,
        value: T,
    ) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::LessThanOrEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_like<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Like(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_not_like<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotLike(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn and_in<T: ToSqlRepr>(self, field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::In(
                field,
                values.into_iter().map(|v| v.to_sql_repr()).collect(),
            ))),
        )
    }

    pub fn and_not_in<T: ToSqlRepr>(self, field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotIn(
                field,
                values.into_iter().map(|v| v.to_sql_repr()).collect(),
            ))),
        )
    }

    pub fn and_between<T: ToSqlRepr>(self, field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Between(
                field,
                min.to_sql_repr(),
                max.to_sql_repr(),
            ))),
        )
    }

    pub fn and_not_between<T: ToSqlRepr>(self, field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotBetween(
                field,
                min.to_sql_repr(),
                max.to_sql_repr(),
            ))),
        )
    }

    pub fn and_is_null(self, field: &'static str) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::IsNull(field))),
        )
    }

    pub fn and_is_not_null(self, field: &'static str) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::IsNotNull(field))),
        )
    }

    pub fn and_raw_condition(self, condition: String) -> WhereClause {
        WhereClause::And(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::RawCondition(
                condition,
            ))),
        )
    }

    pub fn or_equals<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Equals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_not_equals<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_greater_than<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::GreaterThan(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_greater_than_or_equals<T: ToSqlRepr>(
        self,
        field: &'static str,
        value: T,
    ) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::GreaterThanOrEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_less_than<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::LessThan(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_less_than_or_equals<T: ToSqlRepr>(
        self,
        field: &'static str,
        value: T,
    ) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::LessThanOrEquals(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_like<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Like(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_not_like<T: ToSqlRepr>(self, field: &'static str, value: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotLike(
                field,
                value.to_sql_repr(),
            ))),
        )
    }

    pub fn or_in<T: ToSqlRepr>(self, field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::In(
                field,
                values.into_iter().map(|v| v.to_sql_repr()).collect(),
            ))),
        )
    }

    pub fn or_not_in<T: ToSqlRepr>(self, field: &'static str, values: Vec<T>) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotIn(
                field,
                values.into_iter().map(|v| v.to_sql_repr()).collect(),
            ))),
        )
    }

    pub fn or_between<T: ToSqlRepr>(self, field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::Between(
                field,
                min.to_sql_repr(),
                max.to_sql_repr(),
            ))),
        )
    }

    pub fn or_not_between<T: ToSqlRepr>(self, field: &'static str, min: T, max: T) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::NotBetween(
                field,
                min.to_sql_repr(),
                max.to_sql_repr(),
            ))),
        )
    }

    pub fn or_is_null(self, field: &'static str) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::IsNull(field))),
        )
    }

    pub fn or_is_not_null(self, field: &'static str) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::IsNotNull(field))),
        )
    }

    pub fn or_raw_condition(self, condition: String) -> WhereClause {
        WhereClause::Or(
            Box::new(self),
            Box::new(WhereClause::Condition(WhereCondition::RawCondition(
                condition,
            ))),
        )
    }
}

impl SqlBuilder for WhereClause {
    fn build(&self) -> Result<String> {
        match self {
            WhereClause::Condition(condition) => condition.build(),
            WhereClause::And(left, right) => {
                let left = match left.as_ref() {
                    WhereClause::Or(_, _) => {
                        format!("({})", left.build()?)
                    }
                    _ => left.build()?,
                };
                let right = match right.as_ref() {
                    WhereClause::Or(_, _) => {
                        format!("({})", right.build()?)
                    }
                    _ => right.build()?,
                };

                Ok(format!("{} AND {}", left, right))
            }
            WhereClause::Or(left, right) => Ok(format!("{} OR {}", left.build()?, right.build()?)),
        }
    }
}

impl SqlBuilder for WhereCondition {
    fn build(&self) -> Result<String> {
        Ok(match self {
            WhereCondition::Equals(field, value) => format!("{} = {}", field, value),
            WhereCondition::NotEquals(field, value) => format!("{} != {}", field, value),
            WhereCondition::GreaterThan(field, value) => format!("{} > {}", field, value),
            WhereCondition::GreaterThanOrEquals(field, value) => format!("{} >= {}", field, value),
            WhereCondition::LessThan(field, value) => format!("{} < {}", field, value),
            WhereCondition::LessThanOrEquals(field, value) => format!("{} <= {}", field, value),
            WhereCondition::Like(field, value) => format!("{} LIKE {}", field, value),
            WhereCondition::NotLike(field, value) => format!("{} NOT LIKE {}", field, value),
            WhereCondition::In(field, values) => format!("{} IN ({})", field, values.join(", ")),
            WhereCondition::NotIn(field, values) => {
                format!("{} NOT IN ({})", field, values.join(", "))
            }
            WhereCondition::Between(field, min, max) => {
                format!("{} BETWEEN {} AND {}", field, min, max)
            }
            WhereCondition::NotBetween(field, min, max) => {
                format!("{} NOT BETWEEN {} AND {}", field, min, max)
            }
            WhereCondition::IsNull(field) => format!("{} IS NULL", field),
            WhereCondition::IsNotNull(field) => format!("{} IS NOT NULL", field),
            WhereCondition::RawCondition(condition) => condition.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_where_clause_basic() {
        let where_clause = WhereClause::equals("id", 1).and_equals("name", "test");
        assert_eq!(where_clause.build().unwrap(), "id = 1 AND name = 'test'");
    }

    #[test]
    fn test_where_clause_and() {
        let where_clause = WhereClause::equals("id", 1)
            .and_equals("name", "test")
            .and_not_equals("age", 18)
            .and_greater_than("height", 180.1)
            .and_greater_than_or_equals("weight", "60")
            .and_less_than("age", 18)
            .and_less_than_or_equals("age", 18)
            .and_like("name", "test")
            .and_not_like("name", "test")
            .and_in("id", vec![1, 2, 3])
            .and_not_in("id", vec![1, 2, 3])
            .and_between("age", 18, 30)
            .and_not_between("age", 18, 30)
            .and_is_null("name")
            .and_is_not_null("name")
            .and_raw_condition("id = 1".to_string());
        assert_eq!(
            where_clause.build().unwrap(),
            "id = 1 AND name = 'test' AND age != 18 AND height > 180.1 AND weight >= '60' AND age < 18 AND age <= 18 AND name LIKE 'test' AND name NOT LIKE 'test' AND id IN (1, 2, 3) AND id NOT IN (1, 2, 3) AND age BETWEEN 18 AND 30 AND age NOT BETWEEN 18 AND 30 AND name IS NULL AND name IS NOT NULL AND id = 1"
        );
    }

    #[test]
    fn test_where_clause_or() {
        let where_clause = WhereClause::equals("id", 1)
            .or_equals("name", "test")
            .or_not_equals("age", 18)
            .or_greater_than("height", 180.1)
            .or_greater_than_or_equals("weight", "60")
            .or_less_than("age", 18)
            .or_less_than_or_equals("age", 18)
            .or_like("name", "test")
            .or_not_like("name", "test")
            .or_in("id", vec![1, 2, 3])
            .or_not_in("id", vec![1, 2, 3])
            .or_between("age", 18, 30)
            .or_not_between("age", 18, 30)
            .or_is_null("name")
            .or_is_not_null("name")
            .or_raw_condition("id = 1".to_string());
        assert_eq!(
            where_clause.build().unwrap(),
            "id = 1 OR name = 'test' OR age != 18 OR height > 180.1 OR weight >= '60' OR age < 18 OR age <= 18 OR name LIKE 'test' OR name NOT LIKE 'test' OR id IN (1, 2, 3) OR id NOT IN (1, 2, 3) OR age BETWEEN 18 AND 30 OR age NOT BETWEEN 18 AND 30 OR name IS NULL OR name IS NOT NULL OR id = 1"
        );
    }

    #[test]
    fn test_where_clause_and_or() {
        let where_clause = WhereClause::equals("name", "Jack").and(
            WhereClause::equals("age", 18)
                .or_equals("age", 20)
                .or_equals("age", 22),
        );

        assert_eq!(
            where_clause.build().unwrap(),
            "name = 'Jack' AND (age = 18 OR age = 20 OR age = 22)"
        );
    }

    #[test]
    fn test_where_clause_or_and() {
        let where_clause = WhereClause::equals("name", "Jack").or(WhereClause::equals("age", 18)
            .and_equals("age", 20)
            .and_equals("age", 22));

        assert_eq!(
            where_clause.build().unwrap(),
            "name = 'Jack' OR age = 18 AND age = 20 AND age = 22"
        );
    }

    #[test]
    fn test_in_struct() {
        struct User {
            id: i32,
            name: String,
            age: i32,
        }
        let user = User {
            id: 1,
            name: "Jack".to_string(),
            age: 18,
        };

        let where_clause = WhereClause::equals("name", user.name.as_str())
            .and_equals("age", user.age)
            .and_equals("id", user.id);
        assert_eq!(
            where_clause.build().unwrap(),
            "name = 'Jack' AND age = 18 AND id = 1"
        );
        assert_eq!(user.name, "Jack");
    }
}
