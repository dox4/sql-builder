use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum Error {
    // error(s) in update statement
    #[error("you should provide fields that need to be updated.")]
    NoUpdateFields,
    #[error("you should provide constraint(s) before you update record(s).")]
    NoUpdateConditions,
    // error(s) in insert statement
    #[error("you should provide field(s) before you add record(s) to insert.")]
    NoInsertFields,
    #[error("you should provide value(s) that be inserted.")]
    NoInsertValues,
    #[error("the number of fields and values are not match, fields: {0}, values: {1}")]
    FieldValueNotMatch(usize, usize),
    // error(s) in delete statement
    #[error("deleting all records from a table is not allowed, you should provide constraint(s).")]
    NoDeleteConditions,
    // error(s) in select statement
    #[error("you should provide field(s) before you call 'desc' or 'asc'.")]
    NoOrderByClause,
}
