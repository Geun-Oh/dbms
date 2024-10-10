
use std::error::Error;

// use std::net::TcpStream;

// use std::io::{BufReader, BufRead};

use dbms::database::Database;
use dbms::sql_parser;

fn main() -> Result<(), Box<dyn Error>> {

	let mut database = Database::new();

	database.create_table("users", vec!["id".to_string(), "name".to_string(), "age".to_string()])?;

    let insert_query = "INSERT INTO users VALUES (1, \"John Doe\", 30)";
    let result = sql_parser::parse_and_execute(insert_query, &mut database)?;
    println!("Insert result: {:?}", result);

    let select_query = "SELECT * FROM users";
    let result = sql_parser::parse_and_execute(select_query, &mut database)?;
    println!("Select result: {:?}", result);
	// run TCP server
	// let listener = TcpListener::bind("127.0.0.1:9876")?;
	// println!("Server is running on 9876...");

	// for stream in listener.incoming() {
	// 	match stream {
	// 		Ok(stream) => {
	// 			std::thread::spawn(move || {
	// 			})
	// 		}	
	// 		Err(e) => {
	// 			eprintln!("연결 실패: {}", e)
	// 		}
	// 	}
	// }

	Ok(())
}

// fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
// 	let mut reader = BufReader::new(&stream);
// 	let mut buffer = String::new();

// 	loop {
// 		buffer.clear();
// 		reader.read_line(&mut buffer)?;

// 		if buffer.trim().is_empty() {
			
// 		}
// 	}
// }