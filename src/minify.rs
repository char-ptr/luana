use std::collections::HashMap;

use full_moon::{visitors::VisitorMut, tokenizer::{Token, TokenReference}};

const VARIABLE_LETTERS:&'static str="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

#[derive(Default)]
pub struct MinifyVisiter {
    variables:HashMap<String,String>,
}
impl VisitorMut for MinifyVisiter {
    fn visit_token_reference(&mut self, node:TokenReference) ->TokenReference {
        let leading = Vec::from_iter(node.leading_trivia().map(|t| {
            let t = t.clone();

            match t.token_type() {
                full_moon::tokenizer::TokenType::Whitespace { characters } => {
                    if characters.contains("\n") || characters.contains("\r") {
                        return Token::new(full_moon::tokenizer::TokenType::Whitespace { characters: " ".into() });
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
        // println!("token_reference_end : {:#?}",leading);
        let mut new_node = TokenReference::new(
            leading,
            node.token().clone(),
            trailing
        );


        new_node
    }

    // fn visit_local_assignment(&mut self, node:full_moon::ast::LocalAssignment) ->full_moon::ast::LocalAssignment {
    //     self.new_var("pog");
    //     println!("var : {:#?}",node.to_string());
    //     node
    // }
    // visit_bl
    // fn visit_var(&mut self, node:full_moon::ast::Var) ->full_moon::ast::Var {
    //     println!("var : {:#?}",node.to_string());
    //     node
    // }

}
impl MinifyVisiter {
    fn new_var(&mut self,old:&str) {
        let offset = self.variables.len();
        let mut var_name = String::new();
        let sets_amount:f32 = ((offset+1) as f32 / VARIABLE_LETTERS.len() as f32).ceil();
        (0..sets_amount as usize).for_each(|_| {
            var_name.push(VARIABLE_LETTERS.chars().nth(offset % VARIABLE_LETTERS.len()).unwrap());
        });
        self.variables.insert(old.to_string(),var_name.clone());
        println!("new var : {}",var_name);
    }
}