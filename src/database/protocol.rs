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

/// Parses a line which consists of multiple commands seperated with semicolons.
/// Example: "GET a; SET b 2; DEL d"
pub fn parse_multi(line: &str) -> Vec<Result<Command, ParseError>> {
    line.split(';').map(str::trim).filter(|s| !s.is_empty()).map(parse_line).collect()
}

/// Executes a command against DB and returns the answer.
/// Free function, not method on DB, since it connects knowlegde of the protocol and DB access
pub fn execute(db: &Db, cmd: Command) -> String {
    match cmd {
        Command::Get(key) => match db.get(&key) {
            Ok(Some(val)) => format!("OK {val}"),
            Ok(None) => "NOT_FOUND".to_string(),
            Err(e) => format!("ERROR {e}")
        },
        Command::Set(key, value) => match db.set(key, value){
            Ok(()) => "OK".to_string(),
            Err(e) => format!("ERROR {e}")
        },
        Command::Del(key) => match db.del(&key) {
            Ok(true) => "OK".to_string(),
            Ok(false) => "NOT_FOUND".to_string(),
            Err(e) => format!("ERROR {e}")
        }
        Command::Unknown(verb) => format!("ERROR unknown command '{verb}'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get() {
        assert_eq!(parse_line("GET foo"), Ok(Command::Get("foo".to_string())));
    }

    #[test]
    fn parses_set() {
        assert_eq!(
            parse_line("SET foo bar"),
            Ok(Command::Set("foo".to_string(), "bar".to_string()))
        );
    }

    #[test]
    fn is_case_insensitive() {
        assert_eq!(parse_line("get foo"), Ok(Command::Get("foo".to_string())));
    }

    #[test]
    fn empty_line_is_error() {
        assert_eq!(parse_line("   "), Err(ParseError::EmptyLine));
    }

    #[test]
    fn set_without_value_is_error() {
        assert_eq!(
            parse_line("SET foo"),
            Err(ParseError::MissingArgument("value"))
        );
    }

    #[test]
    fn parses_multiple_commands() {
        let results = parse_multi("GET a; SET b 2; DEL d");
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Ok(Command::Get("a".to_string())));
        assert_eq!(
            results[1],
            Ok(Command::Set("b".to_string(), "2".to_string()))
        );
        assert_eq!(results[2], Ok(Command::Del("d".to_string())));
    }

    #[test]
    fn parse_multi_ignores_trailing_semicolon() {
        let results = parse_multi("GET a;");
        assert_eq!(results.len(), 1);
    }
}