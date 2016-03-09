use std::collections::HashMap;


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

    fn parse(&mut self) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn render(&self) -> String {
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
