use crate::lexer::*;

#[derive(Debug)]
enum Node {
    Empty,
    Expression,
    Identifier {
        name: String,
    },
    Assignment {
        variable: String,
        expression: Box<Node>,
    },
    Temporaries {
        variables: Vec<String>,
    },
}
