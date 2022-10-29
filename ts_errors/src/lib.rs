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
    ($errs:expr,$e:expr,$empty:ident) => {
        match $e {
            Ok(t) => t,
            Err(e) => {
                $errs.push(e);
                return $empty::Empty;
            }
        }
    };
    ($e:expr,$empty:ident) => {{
        let e = $e;
        if e.is_empty() {
            return $empty::Empty;
        } else {
            e
        }
    }};
}

#[macro_export]
macro_rules! ptry {
    ($self:expr,$e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => {
                $self.errors.push(e);
                return None;
            }
        }
    };
    ($e:expr) => {{
        if let Some(e) = $e {
            e
        } else {
            return None;
        }
    }};
}

#[macro_export]
macro_rules! pexpect {
    ($self:expr,$e:expr) => {
        ptry!($self, $self.expect($e, line!()))
    };
    ($self:expr,$($x:expr),+ $(,)?) => (
        ptry!($self, $self.expect_multi(&vec![$($x),+], line!()))
    );
}
