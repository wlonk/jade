pub mod parser;


use std::collections::HashMap;


// This is over-simplistic for a context, but it'll do for a first pass.
type Context = HashMap<String, String>;


enum Doctype {
    Html5,
    Xml,
}


struct Node {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<Node>,
}


struct Jade {
    doctype: Doctype,
    root: Node,
}

impl Jade {
    fn new() -> Self {
        unimplemented!()
    }

    fn parse<S: ?Sized>(&mut self, input: &S) -> Result<(), &'static str> 
        where S: AsRef<str>
    {
        unimplemented!()
    }

    fn render(&self, context: Context) -> String {
        unimplemented!()
    }
}


mod test {
    use super::*;

    #[test]
    fn test_jade__new() {
        assert!(true);
    }
}
