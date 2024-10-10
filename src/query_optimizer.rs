use crate::database::Database;
use std::error::Error;

#[derive(Debug)]
pub enum ExecutionStep {
    Scan(String),
    Filter(String),
    Project(Vec<String>),
    Insert(String, Vec<String>),
}

pub struct Optimizer {
    database: Database,
}

impl Optimizer {
    pub fn new(database: Database) -> Self {
        Optimizer { database }
    } 

    pub fn optimize(&self, query: &str) -> Vec<ExecutionStep> {
        if query.to_lowercase().starts_with("select") {
            let table_name = query.split_whitespace().nth(3).unwrap_or("").to_string();
            vec![
                ExecutionStep::Scan(table_name.clone()),
                ExecutionStep::Project(vec!["*".to_string()]),
            ]
        } else if query.to_lowercase().starts_with("insert") {
            let parts: Vec<&str> = query.split_whitespace().collect();
            let table_name = parts[2].to_string(); 
            let values = parts[4].trim_matches(|c| c == '(' || c == ')').split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            vec![ExecutionStep::Insert(table_name, values)]
        } else {
            vec![]
        }
    }

    pub fn execute_plan(&mut self, plan: &[ExecutionStep]) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let mut result = Vec::new();

        for step in plan {
            match step {
                ExecutionStep::Scan(table_name) => {
                    let table = self.database.get_table(table_name);
                    result = table.rows.clone();
                }
                ExecutionStep::Project(columns) => {
                    if columns[0] == "*" {

                    } else {}
                }
                ExecutionStep::Insert(table_name, values) => {
                    let table = self.database.get_table_mut(table_name);
                    table.insert(values.clone()).expect(&format!("Insert Operation Failed"));
                }
                ExecutionStep::Filter(_) => {

                }
            }
        }

        Ok(result)
    }
}