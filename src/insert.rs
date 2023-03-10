use crate::Result;
use crate::{error, SqlBuilder};

#[derive(Debug, Clone)]
pub struct InsertQuery {
    table: &'static str,
    columns: Vec<&'static str>,
    values: Vec<String>,
}

impl InsertQuery {
    pub fn new(table: &'static str) -> InsertQuery {
        InsertQuery {
            table,
            columns: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: &'static str) -> &mut InsertQuery {
        self.columns.push(column);
        self
    }

    pub fn add_columns<I>(&mut self, columns: I) -> &mut InsertQuery
    where
        I: Iterator<Item = &'static str>,
    {
        self.columns.extend(columns);
        self
    }

    pub fn add_record_raw(&mut self, record: &[String]) -> Result<&mut InsertQuery> {
        if self.columns.is_empty() {
            return Err(error::Error::NoInsertFields);
        }
        if !self.columns.is_empty() && self.columns.len() != record.len() {
            return Err(error::Error::FieldValueNotMatch(
                self.columns.len(),
                record.len(),
            ));
        }
        let values = record.join(", ");
        self.values.push(format!("({})", values));
        Ok(self)
    }

    pub fn add_record_from<T: AsInsertRecord>(&mut self, record: &T) -> Result<&mut InsertQuery> {
        let vec = record.as_insert_record();
        if !self.columns.is_empty() && self.columns.len() != vec.len() {
            return Err(error::Error::FieldValueNotMatch(
                self.columns.len(),
                vec.len(),
            ));
        }
        let values = record.as_insert_record().join(", ");
        self.values.push(format!("({})", values));
        Ok(self)
    }
}

impl SqlBuilder for InsertQuery {
    fn build(&self) -> Result<String> {
        if self.columns.is_empty() {
            return Err(error::Error::NoInsertFields);
        }

        if self.values.is_empty() {
            return Err(error::Error::NoInsertValues);
        }
        let columns = self.columns.join(", ");
        let values = self.values.join(", ");
        Ok(format!(
            "INSERT INTO {} ({}) VALUES {}",
            self.table, columns, values
        ))
    }
}

pub trait AsInsertRecord {
    fn as_insert_record(&self) -> Vec<String>;
}

#[cfg(test)]
mod test {
    use super::super::repr::ToSqlRepr;
    use super::*;

    #[test]
    fn test_insert() {
        let mut query = InsertQuery::new("test");
        query
            .add_column("id")
            .add_column("name")
            .add_record_raw(&vec!["1".to_string(), "'test'".to_string()])
            .unwrap()
            .add_record_raw(&vec!["2".to_string(), "'test2'".to_string()])
            .unwrap();
        assert_eq!(
            query.build().unwrap(),
            "INSERT INTO test (id, name) VALUES (1, 'test'), (2, 'test2')"
        );
    }

    #[test]
    fn test_insert_from() {
        let mut query = InsertQuery::new("test");
        query.add_column("id").add_column("name");
        query
            .add_record_from(&TestRecord {
                id: 1,
                name: "test",
            })
            .unwrap()
            .add_record_from(&TestRecord {
                id: 2,
                name: "test2",
            })
            .unwrap();
        assert_eq!(
            query.build().unwrap(),
            "INSERT INTO test (id, name) VALUES (1, 'test'), (2, 'test2')"
        );
    }

    #[derive(Debug, Clone)]
    struct TestRecord {
        id: i32,
        name: &'static str,
    }

    impl AsInsertRecord for TestRecord {
        fn as_insert_record(&self) -> Vec<String> {
            vec![self.id.to_sql_repr(), self.name.to_sql_repr()]
        }
    }

    #[test]
    fn test_no_insert_fields() {
        let mut query = InsertQuery::new("test");
        let err = query
            .add_record_raw(&vec!["1".to_string(), "'test'".to_string()])
            .unwrap_err();
        assert_eq!(err, error::Error::NoInsertFields);
    }

    #[test]
    fn test_no_insert_fields2() {
        let query = InsertQuery::new("test");
        assert_eq!(query.build().unwrap_err(), error::Error::NoInsertFields);
    }

    #[test]
    fn test_no_insert_values() {
        let mut query = InsertQuery::new("test");
        query.add_column("id").add_column("name");
        assert_eq!(query.build().unwrap_err(), error::Error::NoInsertValues);
    }

    #[test]
    fn test_field_value_not_match() {
        let mut query = InsertQuery::new("test");
        query.add_column("id").add_column("name");
        let err = query.add_record_raw(&vec!["1".to_string()]).unwrap_err();
        assert_eq!(err, error::Error::FieldValueNotMatch(2, 1));
    }
}
