#[derive(Debug)]
#[non_exhaustive]
pub enum SPError {
    Reqwest(reqwest::Error),
    Validation(String),
}

impl SPError {
    pub fn unwrap_reqwest(self) -> reqwest::Error {
        match self {
            SPError::Reqwest(err) => err,
            SPError::Validation(msg) => panic!("Called unwrap_reqwest on a Validation error: {}", msg),
        }
    }
    
    pub fn is_reqwest(&self) -> bool {
        matches!(self, SPError::Reqwest(_))
    }
    
    pub fn is_validation(&self) -> bool {
        matches!(self, SPError::Validation(_))
    }
}