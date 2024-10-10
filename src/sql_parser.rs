use pest::Parser;
use pest_derive::Parser;
use std::error::Error;

use crate::database::Database;
use crate::query_optimizer::Optimizer;

#[derive(Parser)]
#[grammar = "sql.pest"] 
pub struct SQLParser;

pub fn parse_and_execute(query: &str, database: &mut Database) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    // let pairs = SQLParser::parse(Rule::sql_grammar, query)
    //     .expect("Failed to parse SQL")
    //     .next()
    //     .unwrap();


    let mut optimizer = Optimizer::new(database.clone());
    let plan = optimizer.optimize(query);

    println!("Execution plan:");
    for step in &plan {
        println!("{:?}", step);
    }

    optimizer.execute_plan(&plan)
}