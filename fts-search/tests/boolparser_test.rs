pub use fts_search::boolparser::*;


#[test]
fn test_lex() {
    let inputs = vec!["cat AND dog OR bird AND (tree OR sun)".to_string(), 
                      "(rain AND sun) OR bird AND (street OR people)".to_string()];
    let outputs = vec![
        vec![
            Token::Name { text: "cat".to_string() }, 
            Token::BinaryOp(BinaryOp::And), 
            Token::Name { text: "dog".to_string() }, 
            Token::BinaryOp(BinaryOp::Or), 
            Token::Name { text: "bird".to_string() },
            Token::BinaryOp(BinaryOp::And), 
            Token::OpenBracket,
            Token::Name { text: "tree".to_string() },
            Token::BinaryOp(BinaryOp::Or), 
            Token::Name { text: "sun".to_string() },
            Token::CloseBracket,
        ],
        vec![
            Token::OpenBracket, 
            Token::Name { text: "rain".to_string() }, 
            Token::BinaryOp(BinaryOp::And), 
            Token::Name { text: "sun".to_string() }, 
            Token::CloseBracket, 
            Token::BinaryOp(BinaryOp::Or), 
            Token::Name { text: "bird".to_string() }, 
            Token::BinaryOp(BinaryOp::And), 
            Token::OpenBracket, 
            Token::Name { text: "street".to_string() }, 
            Token::BinaryOp(BinaryOp::Or), 
            Token::Name { text: "people".to_string() }, 
            Token::CloseBracket
        ]
    ];

    for (i, inp) in inputs.iter().enumerate() {
        let tokens = lex(inp.to_string());
        assert_eq!(tokens, outputs[i]);
    }
}

#[test]
fn test_simple_munch() {
    let inputs = vec!["cat AND dog".to_string(), "NOT dog".to_string()];
    let outputs = vec![
        Box::new(ASTNode::Binary(BinaryOp::And, 
            Box::new(ASTNode::Name("cat".to_string())),
            Box::new(ASTNode::Name("dog".to_string())))
        ),
        Box::new(ASTNode::Invert(Box::new(ASTNode::Name("dog".to_string()))))
    ];

    for (i, inp) in inputs.iter().enumerate() {
        let tokens = lex(inp.to_string());
        let ast  = munch_tokens(tokens);
        match ast {
            Some(out) => assert_eq!(out, outputs[i]),
            None => assert!(false)
        }
    }
}


#[test]
fn test_fail_munch() {
    let inputs = vec!["AND dog", "dog AND", "dog OR"];
    for (_, inp) in inputs.iter().enumerate() {
        let tokens = lex(inp.to_string());
        let ast  = munch_tokens(tokens);
        match ast {
            Some(_out) => assert!(false),
            None => assert!(true)
        }
    }
}


#[test]
fn test_recurs_munch() {
    let inputs = vec![
                      "cat AND dog OR bird".to_string(), 
                      "cat AND dog OR (bird AND mouse)".to_string(),
                      "rain AND sun OR bird AND (street OR people)".to_string(),
                      "(rain AND sun) OR bird AND (street OR people)".to_string()
                      ];
    let outputs = vec![
        Box::new(ASTNode::Binary(BinaryOp::Or, 
            Box::new(ASTNode::Binary(BinaryOp::And, 
                Box::new(ASTNode::Name("cat".to_string())),
                Box::new(ASTNode::Name("dog".to_string())),
            )),
            Box::new(ASTNode::Name("bird".to_string())))
        ),
        Box::new(ASTNode::Binary(BinaryOp::Or, 
            Box::new(ASTNode::Binary(BinaryOp::And, 
                Box::new(ASTNode::Name("cat".to_string())),
                Box::new(ASTNode::Name("dog".to_string())),
            )),
            Box::new(ASTNode::Binary(BinaryOp::And, 
                Box::new(ASTNode::Name("bird".to_string())), 
                Box::new(ASTNode::Name("mouse".to_string())))))),
        Box::new(ASTNode::Binary(BinaryOp::And, 
            Box::new(ASTNode::Binary(BinaryOp::Or, 
                Box::new(ASTNode::Binary(BinaryOp::And, 
                    Box::new(ASTNode::Name("rain".to_string())), 
                    Box::new(ASTNode::Name("sun".to_string())))), 
                Box::new(ASTNode::Name("bird".to_string())))), 
            Box::new(ASTNode::Binary(BinaryOp::Or, 
                Box::new(ASTNode::Name("street".to_string())), 
                Box::new(ASTNode::Name("people".to_string())))))),
        Box::new(ASTNode::Binary(BinaryOp::And, 
            Box::new(ASTNode::Binary(BinaryOp::Or, 
                Box::new(ASTNode::Binary(BinaryOp::And, 
                    Box::new(ASTNode::Name("rain".to_string())), 
                    Box::new(ASTNode::Name("sun".to_string())))), 
                Box::new(ASTNode::Name("bird".to_string())))), 
            Box::new(ASTNode::Binary(BinaryOp::Or, 
                Box::new(ASTNode::Name("street".to_string())), 
                Box::new(ASTNode::Name("people".to_string()))))))
    ];

    for (i, inp) in inputs.iter().enumerate() {
        let tokens = lex(inp.to_string());
        let ast  = munch_tokens(tokens);
        match ast {
            Some(out) => assert_eq!(out, outputs[i]),
            None => {
                assert!(false)
            }
        }
    }
}
