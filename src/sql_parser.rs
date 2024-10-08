use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"] // .pest defines grammar of SQL 
pub struct SQLParser;

#[derive(Debug)]
enum ExecutionStep {
    Scan(String),
    Filter(String),
    Project(Vec<String>),
    Insert(String, Vec<String>, Vec<String>),
}

struct Optimizer {
    table_stats: HashMap<String, usize>
}

impl Optimizer {
    fn new() -> Self {
        Optimizer {
            table_stats: HashMap::new(),
        }
    }

    fn optimize(&self, query: &str) -> Vec<ExecutionStep> {
        let pairs = SQLParser::parse(Rule::sql_grammar, query)
            .expect("Failed to parse SQL")
            .next().unwrap();

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::select_stmnt => return,
                Rule::select_stmnt => return,
                _ => panic!("Unsupported statement type"),
            }
        }

        vec![]
    }

    fn optimize_select(&self, select_pair: pest::iterators::Pair<Rule>) -> Vec<ExecutionStep> {
        let mut plan = Vec::new();
        let mut table_name = String::new();

        for pair in select_pair.into_inner() {
            match pair.as_rule() {
                Rule::table_name => {
                    table_name = pair.as_str().to_string();
                    plan.push(ExecutionStep::Scan(table_name::clone()));
                }
                Rule::star => {
                    plan.push(ExecutionStep::Project(vec!["*".to_string()]));
                }
                _ => {}
            }
        }

        plan
    }
}