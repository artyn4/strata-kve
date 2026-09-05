use strata_kve::engine::{Engine, Options};
use std::io::{self, Write};

fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c.is_whitespace() && !in_quotes {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

fn main() {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ strata-kve cli                                          │");
    println!("│ type commands or you can type 'exit' to escape          │");
    println!("└─────────────────────────────────────────────────────────┘");

    let options = Options {
        db_path: "./data_trash".into(),
        memtable_size_bytes: 4 * 1024 * 1024,
        block_cache_capacity_bytes: 64 * 1024 * 1024,
    };

    let mut dbz = Engine::open(options).unwrap();
    println!("dbz initialized successfully");

    loop {
        print!("strata_yay> ");
        io::stdout().flush().unwrap();

        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("stdin gave up");
        let cmd = input_str.trim();

        if cmd.is_empty() {
            continue;
        }

        println!("input was: {:?}", cmd); // forgot to remove debug print

        let parts = parse_args(cmd);
        let action = parts[0].as_str();

        match action {
            "exit" => {
                println!("you successfully left");
                break;
            }
            "put" => {
                if parts.len() < 3 {
                    println!("our format is: put <key> \"<value>\"");
                    continue;
                }
                let k = parts[1].as_bytes();
                let v = parts[2].as_bytes();
                dbz.put(k, v).unwrap();
                println!("your data is saved");
            }
            "get" => {
                if parts.len() < 2 {
                    println!("give a key please -> get <key>");
                    continue;
                }
                let k = parts[1].as_bytes();
                let res = dbz.get(k).unwrap();
                match res {
                    Some(val) => println!("GOT IT: {}", String::from_utf8_lossy(&val)),
                    None => println!("oops key not found"),
                }
            }
            "trace" => {
                if parts.len() < 2 {
                    println!("we need a key to trace");
                    continue;
                }
                let target = &parts[1];
                let k = target.as_bytes();

                let start = std::time::Instant::now();
                let result = dbz.get(k).unwrap();
                let duration = start.elapsed();

                println!("┌─────────────────────────────────────────────────────────────────────────┐");
                println!("│  TRACE FOR KEY: '{}'                                                    │", target);
                println!("├─────────────────────────────────────────────────────────────────────────┤");
                println!("│ Step 1: [memTable / and scan]");
                println!("│    Outcome  : {}", if result.is_some() { "hit" } else { "miss" });
                println!("│    Duration : {:.2?}", duration);
                println!("├─────────────────────────────────────────────────────────────────────────┤");
                println!("│ Summary: found={} | time={:.2?}", result.is_some(), duration);
                println!("└─────────────────────────────────────────────────────────────────────────┘");
            }
            "stats" => {
                println!("=== STATS ===");
                println!("active sstables: {}", dbz.tables.len());
                println!("memtable bytes: {}", dbz.memz.estimated_size());
            }
            "help" => {
                println!("commands: put <k> \"<v>\" | get <k> | trace <k> | stats | exit");
            }
            _ => {
                println!(" '{}' you cant leave it empty", action);
            }
        }
    }
}
