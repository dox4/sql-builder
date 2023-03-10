pub mod delete;
pub mod error;
pub mod insert;
pub mod repr;
pub mod select;
pub mod update;
pub mod where_clause;


use anyhow;
type Result<T> = anyhow::Result<T, crate::error::Error>;

pub trait SqlBuilder {
    fn build(&self) -> Result<String>;
}
