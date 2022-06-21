use std::fmt;

#[derive(Debug, Clone)]
pub struct LexError {
    pub error: String,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl LexError {
    pub fn new(error: String) -> Self {
        LexError { error }
    }
}

#[derive(Debug)]
pub struct ParseError {
    error: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ParseError {
    pub fn new(error: &String) -> Self {
        ParseError {
            error: error.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CodeGenError {
    pub message: String,
}


#[macro_export]
macro_rules! check {
    ($self:ident,$e:expr,$empty:ident) => {
        match $e {
            Ok(t) => t,
            Err(e) => {
                $self.errors.borrow_mut().push(e);
                return $empty::Empty;
            }
        }
    };
    ($e:expr) => {{
        let e = $e;
        if e.is_empty() {
            return Value::Empty;
        } else {
            e
        }
    }};
}
