use super::db::Db;
use std::fmt;

/// Paired command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Unknown(String)
}

/// Handle errors that could occur when parsing a protocol line
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyLine,
    MissingArgument(&'static str)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "Empty input"),
            ParseError::MissingArgument(what) => write!(f, "Missing argument: {what}")
        }
    }
}

// Implementing so errors are treated liek actual Rust errors, compatible with '?'
impl std::error::Error for ParseError {}

/// Parse single protocol line to a command
pub fn parse_line(line: &str) -> Result<Command, ParseError> {
    let mut parts = line.trim().split_whitespace();
    let verb = parts.next().ok_or(ParseError::EmptyLine)?;

    match verb.to_uppercase().as_str() {
        "GET" => {
            let key = parts.next().ok_or(ParseError::MissingArgument("key"))?;
            Ok(Command::Get(key.to_string()))
        }
        "SET" => {
            let key = parts.next().ok_or(ParseError::MissingArgument("key"))?;
            let value = parts.next().ok_or(ParseError::MissingArgument("value"))?;
            Ok(Command::Set(key.to_string(), value.to_string()))
        }
        "DEL" => {
            let key = parts.next().ok_or(ParseError::MissingArgument("key"))?;
            Ok(Command::Del(key.to_string()))
        }
        other => Ok(Command::Unknown(other.to_string()))
    }
}

/// Executes a command against DB and returns the answer.
/// Free function, not method on DB, since it connects knowlegde of the protocol and DB access
pub fn execute(db: &Db, cmd: Command) -> String {
    match cmd {
        Command::Get(key) => match db.get(&key) {
            Some(value) => format!("Ok {value}"),
            None => "NOT FOUND".to_string()
        },
        Command::Set(key, value) => {
            db.set(key, value);
            "OK".to_string()
        },
        Command::Del(key) => {
            if db.del(&key) {
                "OK".to_string()
            } else {
                "NOT_FOUND".to_string()
            }
        }
        Command::Unknown(verb) => format!("ERROR unknown command '{verb}'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get() {
        assert_eq!(parse_line("GET foo"),
        Ok(Command::Get("foo".to_string())));
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse_line("SET foo bar"),
            Ok(Command::Set("foo".to_string(), "bar".to_string()))
        );
    }

    #[test]
    fn is_case_insensitive() {
        assert_eq!(parse_line("get foo"),
        Ok(Command::Get("foo".to_string())));
    }

    #[test]
    fn empty_line_is_error() {
        assert_eq!(parse_line("   "), Err(ParseError::EmptyLine));
    }

    #[test]
    fn set_without_value_is_error() {
        assert_eq!(parse_line("SET foo"), Err(ParseError::MissingArgument("value")));
    }
}