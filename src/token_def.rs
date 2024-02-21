/*
use crate::token::Token;

macro_rules! token {
    ($name:ident) => {
        pub struct $name {
            text: String,
            pos: i32,
            line: i32,
        }

        impl $name {
            pub fn new(text: String, pos: i32, line: i32) -> Self {
                Self{
                    text: text,
                    pos: pos,
                    line: line
                }
            }
        }

        impl Token for $name {
            fn from_str(&self, text: String, pos: i32, line: i32) -> Box<dyn Token> {
                Box::new($name::new(text, pos, line))
            }
        }
    };
}

token!(Plus);
*/