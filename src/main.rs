use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};

// TCP server get va post
// Memcached <-, hotirada saqlaymiz
// Persistance ?

// SET
// <key>
// <value>

// GET
// <key>

// LIST

#[derive(Debug)]
enum Operation {
    GET { key: String },
    SET { key: String, value: String },
    LIST,
}

fn handle_client(
    mut stream: TcpStream,
    memory: &mut HashMap<String, String>,
) -> Result<(), String> {
    println!("{:?}", stream);
    let buffer = BufReader::new(&stream);
    let mut raw_lines = buffer
        .lines()
        .map(|line| match line {
            Ok(str) => str,
            _ => String::new(),
        })
        .take_while(|line| !line.is_empty())
        .collect::<VecDeque<String>>();

    println!("{:?}", &raw_lines);

    let operation = raw_lines
        .pop_front()
        .ok_or("Failed to parse your request")?;
    let operation: Operation = if operation.eq("GET") {
        let key = raw_lines.pop_front().ok_or("Incomplete request")?;
        Operation::GET { key: key.unwrap() }
    } else if operation.eq("SET") {
        let key = raw_lines.pop_front().ok_or("Incomplete request")?;
        let value = raw_lines.pop_front().ok_or("Incomplete request")?;
        Operation::SET { key, value }
    } else if operation.eq("LIST") {
        Operation::LIST
    } else {
        return Err("Unknown operation".to_string());
    };

    match operation {
        Operation::GET { key } => {
            if let Some(value) = memory.get(&key) {
                let _ = stream.write(value.as_bytes());
            } else {
                let _ = stream.write(&[]);
            }
        }
        Operation::SET { key, value } => {
            let _ = memory.insert(key, value);
        }
        Operation::LIST => {
            let kvs = memory
                .iter()
                .map(|(k, v)| format!("{},{}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n");
            let _ = stream.write(kvs.as_bytes());
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:1256").expect("Failed to listen on port 1256");
    println!("{:?}", listener);

    let mut memory: HashMap<String, String> = HashMap::new();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream, &mut memory).ok();
        } else if let Err(error) = stream {
            println!("Weird error happend: {:?}", error);
        }
    }

    Ok(())
}
