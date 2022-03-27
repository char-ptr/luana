use full_moon::{visitors::VisitorMut, tokenizer::{Token, TokenReference}};
#[derive(Default)]
pub struct MinifyVisiter {

}
impl VisitorMut for MinifyVisiter {
    // fn visit_token(&mut self, token:full_moon::tokenizer::Token) ->full_moon::tokenizer::Token {
    //     // println!("token : {:?}",token);
    //     let mut new_token = token.clone();
    //     match token.token_type() {
    //         full_moon::tokenizer::TokenType::Whitespace { characters } => {

    //             if characters.contains("\n"){
    //                     new_token = Token::new(full_moon::tokenizer::TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::Semicolon })
    //             } 
    //             else if !characters.is_empty() {
    //                     new_token = Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: " ".into() })
    //                 }
    //         },
    //         _ => {},
    //     }
    //     new_token
    // }
    
    // fn visit_prefix_end(&mut self, node:full_moon::ast::Prefix) ->full_moon::ast::Prefix {
    //     // println!("suffix_end : {:#?}",node);
    //     let mut new_node = node.clone();
    //     // println!("pref : {:?}",new_node);
    //     match node {
    //         full_moon::ast::Prefix::Expression(_) => todo!(),
    //         full_moon::ast::Prefix::Name(n) => {
    //             // let leading = Vec::from_iter(n.leading_trivia().filter(|t| {
    //             //     match t.token_type() {
    //             //         full_moon::tokenizer::TokenType::Symbol { symbol } => {
    //             //             if symbol == &full_moon::tokenizer::Symbol::Semicolon {
    //             //                 return false;
    //             //             }
    //             //             true
    //             //         },
    //             //         _ => true,
    //             //     }
    //             // }).map(|t| t.clone()));
    //             let new_ref = TokenReference::new(vec![], Token::new(n.token().token_type().clone()), Vec::from_iter(n.trailing_trivia().map(|t| t.clone())));
    //             new_node=full_moon::ast::Prefix::Name(new_ref);
    //             // println!("prefA : {:#?}",new_node);
    //         },
    //         _ => {},
    //     }

        // new_node
    // }
    fn visit_token_reference(&mut self, node:TokenReference) ->TokenReference {
        let leading = Vec::from_iter(node.leading_trivia().map(|t| {
            let t = t.clone();

            match t.token_type() {
                full_moon::tokenizer::TokenType::Whitespace { characters } => {
                    if characters.contains("\n") || characters.contains("\r") {
                        return Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: "".into() });
                    } else if !characters.is_empty() {
                        return Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: "".into() });
                    }
                    t
                },
                _ => t,
            }
        }));
        let trailing = Vec::from_iter(node.trailing_trivia().map(|t| {
            let t = t.clone();

            match t.token_type() {
                full_moon::tokenizer::TokenType::Whitespace { characters } => {
                    if characters.contains("\n") || characters.contains("\r") {
                        return Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: " ".into() });
                    } else if !characters.is_empty() {
                        return Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: " ".into() });
                    }
                    t
                },
                _ => t,
            }
        }));
        println!("token_reference_end : {:#?}",leading);
        let mut new_node = TokenReference::new(
            leading,
            node.token().clone(),
            trailing
        );


        new_node
    }
}