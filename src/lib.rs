pub mod delete;
pub mod insert;
pub mod select;
pub mod update;
pub mod where_clause;

pub trait SqlBuilder {
    fn build(&self) -> String;
}
