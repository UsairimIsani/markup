use clap::{load_yaml, App};

mod parser;
use parser::Parser;
fn main() {
    let yaml = load_yaml!("markup.yaml");
    let matches = App::from(yaml).get_matches();
    let parser = Parser::new(matches.value_of("INPUT").unwrap());
    for i in parser.content {
        println!("{}", i.val);
    }
}
