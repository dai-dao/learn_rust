

// Box is just pointers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    Invert(Box<ASTNode>),
    Binary(BinaryOp, Box<ASTNode>, Box<ASTNode>),
    Name(String)
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Name { text : String },
    BinaryOp(BinaryOp),
    InvertOp,
    OpenBracket,
    CloseBracket
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    And,
    Or
}


// need to go from right to left
pub fn munch_tokens(tokens: Vec<Token>) -> Option<Box<ASTNode>> {
    let mut op_token : Option<BinaryOp> = None;
    let mut invert_token = false;
    let mut right_node : Option<Box<ASTNode>> = None;
    let mut left_node : Option<Box<ASTNode>> = None;

    for token in tokens.iter().rev() {
        match token {
            Token::Name{ text } => {
                match right_node {
                    Some(_) => left_node = Some(Box::new(ASTNode::Name(text.to_string()))),
                    None => right_node = Some(Box::new(ASTNode::Name(text.to_string()))), 
                }
            }
            Token::BinaryOp(BinaryOp::And) => {
                match op_token {
                    // Binary op already exists, recurse further down the tree
                    Some(_) => {
                        let sub_tokens = (&tokens[..tokens.len()-2]).to_vec();
                        left_node = munch_tokens(sub_tokens);
                        break;
                    }
                    None => op_token = Some(BinaryOp::And),
                }
            }
            Token::BinaryOp(BinaryOp::Or) => {
                match op_token {
                    // Binary op already exists, recurse further down the tree
                    Some(_) => {
                        let sub_tokens = (&tokens[..tokens.len()-2]).to_vec();
                        left_node = munch_tokens(sub_tokens);
                        break;
                    }
                    None => op_token = Some(BinaryOp::Or),
                }
            }
            Token::InvertOp => invert_token = true,
            _ => (),
        }
    }


    // Assume that it can only be BinaryOp or InvertOp
    if let Some(bin_op) = op_token {
        if let (Some(left), Some(right)) = (left_node, right_node) {
            return Some(Box::new(ASTNode::Binary(bin_op, left, right)))
        } else {
            return None
        }
    } else if invert_token {
        return Some(Box::new(ASTNode::Invert(right_node.unwrap())))
    } else {
        return None
    }
}


fn parse_token(builder : &Vec<char>) -> Token {
    let word = builder.into_iter().collect::<String>();
    return match &(*word) {
        "AND" => Token::BinaryOp(BinaryOp::And),
        "OR" => Token::BinaryOp(BinaryOp::Or),
        "NOT" => Token::InvertOp,
        _ => Token::Name { text : word }
    }
}

 
pub fn lex(query : String) -> Vec<Token> {
    let mut out : Vec<Token> = Vec::new();
    let mut builder = Vec::new();

    for (i, c) in query.chars().enumerate() {
        match c {
            c if c.is_whitespace() => {
                out.push(parse_token(&builder));
                builder.clear();
            }
            '(' => out.push(Token::OpenBracket),
            ')' => {
                // last word
                if i == query.len()-1 && builder.len() > 0 {
                    out.push(parse_token(&builder));
                    builder.clear();
                }
                out.push(Token::CloseBracket);
            }
            _ => builder.push(c)
        }
    }
    // last word
    if builder.len() > 0 {
        out.push(parse_token(&builder));
        builder.clear();
    }

    return out
}