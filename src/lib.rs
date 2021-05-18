use serde_json::Value;

#[derive(Hash, PartialEq, Clone, Debug)]
pub enum Error {
    Http(String),
    Other,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Http(error) => {
                write!(f, "Error Getting Kanye: {}", error)
            }
            Error::Other => {
                write!(f, "Other Error")
            }
        }
    }
}


pub fn quote() -> Result<String, Error> {
    let response = match ureq::get("https://api.kanye.rest").call() {
        Ok(data) => data,
        Err(error) => return Err(Error::Http(error.to_string())),
    };

    let data = match response.into_string() {
        Ok(data) => data,
        Err(_) => return Err(Error::Other),
    };

    let v: Value = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(_) => return Err(Error::Other),
    };

    let output = match &v["quote"] {
        Value::String(output) => output.to_string(),
        _ => return Err(Error::Other),
    };

    return Ok(output);
}