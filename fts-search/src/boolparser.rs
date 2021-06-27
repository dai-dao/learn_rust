

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
    let mut op_token_index = 0;
    let mut invert_token = false;
    let mut right_node : Option<Box<ASTNode>> = None;
    let mut left_node : Option<Box<ASTNode>> = None;
    let mut close_bracket = 0;
    let mut close_bracket_index = 0;

    for (i, token) in tokens.iter().rev().enumerate() {
        match token {
            Token::OpenBracket => {
                // make right node the result of bracket expression, from the close bracket to open bracket
                // move iteration cursor to after the open bracket and then continue
                if close_bracket > 0 {
                    close_bracket -= 1;
                    if close_bracket == 0 {
                        let current_index = tokens.len() - i; // exclude the bracket
                        let sub_tokens = (&tokens[current_index..close_bracket_index]).to_vec();
                        match right_node {
                            Some(_) => left_node = munch_tokens(sub_tokens),
                            None => right_node = munch_tokens(sub_tokens),
                        }
                    }
                }
            }
            Token::CloseBracket => {
                if close_bracket == 0 {
                    close_bracket_index = tokens.len() - i - 1;
                }
                close_bracket += 1;
            }
            Token::Name{ text } => {
                // in case just one token
                // if 

                if close_bracket == 0 {
                    match right_node {
                        Some(_) => left_node = Some(Box::new(ASTNode::Name(text.to_string()))),
                        None => right_node = Some(Box::new(ASTNode::Name(text.to_string()))), 
                    }
                }
            }
            Token::BinaryOp(BinaryOp::And) => {
                if close_bracket == 0 {
                    match op_token {
                        // Binary op already exists, recurse further down the tree
                        Some(_) => {
                            let sub_tokens = (&tokens[..op_token_index]).to_vec();
                            left_node = munch_tokens(sub_tokens);
                            break;
                        }
                        None => {
                            op_token = Some(BinaryOp::And);
                            op_token_index = tokens.len() - i - 1;
                        }
                    }
                }
            }
            Token::BinaryOp(BinaryOp::Or) => {
                if close_bracket == 0 {
                    match op_token {
                        // Binary op already exists, recurse further down the tree
                        Some(_) => {
                            let sub_tokens = (&tokens[..op_token_index]).to_vec();
                            left_node = munch_tokens(sub_tokens);
                            break;
                        }
                        None => {
                            op_token = Some(BinaryOp::Or);
                            op_token_index = tokens.len() - i - 1;
                        }
                    }
                }
            }
            Token::InvertOp => if close_bracket == 0 { invert_token = true },
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
        return right_node
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

    for c in query.chars() {
        match c {
            c if c.is_whitespace() => {
                if builder.len() > 0 {
                    out.push(parse_token(&builder));
                    builder.clear();
                }
            }
            '(' => out.push(Token::OpenBracket),
            ')' => {
                // last word
                if builder.len() > 0 {
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