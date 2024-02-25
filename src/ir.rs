use std::fmt::Debug;


//#[derive(Debug)]
pub enum IRInstruction {
    Add{left: i64, right: i64},
    Sub{left: i64, right: i64},
    Mult{left: i64, right: i64},
    Div{left: i64, right: i64},
}

impl Debug for IRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
