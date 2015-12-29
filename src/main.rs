mod tokenizer;
mod executor;
use tokenizer::*;
use executor::execute;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn main() {
    // contain all program variables
    let mut variables: HashMap<String, f32> = HashMap::new();
    // string to execute
    let mut buffer = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()
            .ok()
            .expect( "[error] Can't flush to stdout!" );
        io::stdin().read_line(&mut buffer)
            .ok()
            .expect( "[error] Can't read line from stdin!" );
        // split string to tokens
        let data = tokenizer::tokenize(buffer.trim());
        // execute operation (check by exit flag)
        if execute(&mut variables, &data) {
            break;
        }
        // clean string
        buffer.clear();
    }
}
