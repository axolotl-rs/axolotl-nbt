mod value;
mod error;
pub(crate) mod lexer;
mod parser;

use logos::Logos;
pub use error::Error;
pub use value::{Value, NameLessValue};
use crate::lexer::Token;

#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::lexer::Token;
    use crate::to_value;

    #[test]
    fn it_works() {
        let value = to_value(r#"{name1:123,name2:"sometext1",name3:{subname1:456,subname2:"sometext2"}}"#).unwrap();
        println!("{:?}", value);
    }
}


pub fn to_value(str: &str) -> Result<Value, Error> {
    let mut lex = Token::lexer(str);
    let value = parser::parse( lex)?;
    Ok(value)
}