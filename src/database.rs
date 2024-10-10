use std::collections::HashMap;
use crate::table::Table;
use std::error::Error;

#[derive(Clone)]
pub struct Database {
	tables: HashMap<String, Table>,
}

impl Database {
	pub fn new() -> Self {
		Database {
			tables: HashMap::new(),
		}
	}

	pub fn create_table(&mut self, table_name: &str, schema: Vec<String>) -> Result<(), Box<dyn Error>> {
		let table = Table::new(table_name, schema).expect(&format!("Creating table with {} is not available", table_name));
		self.tables.insert(table_name.to_string(), table);
		Ok(())
	}

	pub fn get_table(&mut self, table_name: &str) -> &Table {
		self.tables.get(table_name).expect(&format!("Table with name {} is not here", table_name))
	}

	pub fn get_table_mut(&mut self, table_name: &str) -> &mut Table {
		self.tables.get_mut(table_name).expect(&format!("Table with name {} is not here", table_name))
	}
}