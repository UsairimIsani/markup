pub struct Parser {
    pub content: Vec<Content>,
}

impl Parser {
    pub fn new(content: &str) -> Self {
        let v = vec![Content::new(content)];
        Self { content: v }
    }
    // pub fn parse(&self)-> Vec<Content> {
    //     unimplemented!
    // }
}

pub struct Content {
    pub val: String,
}

impl Content {
    pub fn new(content: &str) -> Self {
        Self {
            val: content.to_string(),
        }
    }
}
