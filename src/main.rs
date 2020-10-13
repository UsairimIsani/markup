use clap::{load_yaml, App};
mod elements;
use elements::heading::Heading;
fn main() {
    let heading = Heading::new("Hello World");
    println!("{}", heading);
    let yaml = load_yaml!("markup.yaml");
    let matches = App::from(yaml).get_matches();
}
