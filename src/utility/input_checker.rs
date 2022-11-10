use crate::utility::simple_regex::SimpleRegex;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum InputError {
    IoError(io::Error),
    ValueOutOfRange {
        line: usize,
        head: usize,
        val: String,
        min: String,
        max: String,
    },
    UnexpectedCharacter {
        line: usize,
        head: usize,
    },
    UnexpectedEof {
        line: usize,
        head: usize,
    },
    UnexpectedEoln {
        line: usize,
        head: usize,
    },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::IoError(ref e) => write!(f, "{}", e),
            InputError::ValueOutOfRange {
                line,
                head,
                val,
                min,
                max,
            } => write!(
                f,
                "line{}:{}: value {} out of range [{}, {}]",
                line, head, val, min, max
            ),
            InputError::UnexpectedCharacter { line, head } => {
                write!(f, "line{}:{}: unexpected character", line, head)
            }
            InputError::UnexpectedEof { line, head } => {
                write!(f, "line{}:{}: unexpected eof", line, head)
            }
            InputError::UnexpectedEoln { line, head } => {
                write!(f, "line{}:{}: unexpected eoln", line, head)
            }
        }
    }
}

impl error::Error for InputError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            InputError::IoError(ref e) => Some(e),
            InputError::ValueOutOfRange {
                line: _,
                head: _,
                val: _,
                min: _,
                max: _,
            } => None,
            InputError::UnexpectedCharacter { line: _, head: _ } => None,
            InputError::UnexpectedEof { line: _, head: _ } => None,
            InputError::UnexpectedEoln { line: _, head: _ } => None,
        }
    }
}

impl From<io::Error> for InputError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[derive(Debug)]
pub struct InputChecker {
    buffer: Vec<char>,
    has_read_eof: bool,
    line: usize,
    head: usize,
}

impl Drop for InputChecker {
    fn drop(&mut self) {
        assert!(self.has_read_eof);
    }
}

impl InputChecker {
    pub fn new() -> InputChecker {
        let buffer = String::new().chars().collect();

        InputChecker {
            buffer,
            has_read_eof: false,
            line: 0,
            head: 0,
        }
    }

    fn read(&mut self) -> Result<(), InputError> {
        if self.head == self.buffer.len() {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            self.buffer = buf.chars().collect();
            if !self.buffer.is_empty() {
                self.line += 1;
                self.head = 0;
            }
        }
        Ok(())
    }

    pub fn read_string(&mut self, regex: SimpleRegex) -> Result<String, InputError> {
        if self.has_read_eof {
            return Err(InputError::UnexpectedEof {
                line: self.line,
                head: self.head,
            });
        }

        self.read()?;
        if self.buffer.is_empty() {
            return Err(InputError::UnexpectedEof {
                line: self.line,
                head: self.head,
            });
        }

        let end = regex.longest_match(&self.buffer, self.head);
        if end == self.head {
            return Err(InputError::UnexpectedCharacter {
                line: self.line,
                head: self.head,
            });
        }

        let res = self.buffer[self.head..end].iter().collect();
        self.head = end;

        Ok(res)
    }

    pub fn read_number<T>(&mut self, min: T, max: T) -> Result<T, InputError>
    where
        T: Ord + std::str::FromStr + Display,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.read()?;
        if self.buffer.is_empty() {
            return Err(InputError::UnexpectedEof {
                line: self.line,
                head: self.head,
            });
        }
        if self.buffer[self.head] == '0' {
            self.head += 1;
            if self.head < self.buffer.len() && self.buffer[self.head].is_ascii_digit() {
                return Err(InputError::UnexpectedCharacter {
                    line: self.line,
                    head: self.head,
                });
            }
            return Ok("0".parse::<T>().unwrap());
        }

        let mut digits = vec![];
        if self.buffer[self.head] == '-' {
            self.head += 1;
            if self.head == self.buffer.len() {
                return Err(InputError::UnexpectedEoln {
                    line: self.line,
                    head: self.head,
                });
            }
            if !self.buffer[self.head].is_ascii_digit() || self.buffer[self.head] == '0' {
                return Err(InputError::UnexpectedCharacter {
                    line: self.line,
                    head: self.head,
                });
            }
            digits.push('-');
        }
        while self.head < self.buffer.len() && self.buffer[self.head].is_ascii_digit() {
            digits.push(self.buffer[self.head]);
            self.head += 1;
        }

        let res = digits.iter().collect::<String>().parse::<T>().unwrap();

        if res < min || max < res {
            return Err(InputError::ValueOutOfRange {
                line: self.line,
                head: self.head,
                val: res.to_string(),
                min: min.to_string(),
                max: max.to_string(),
            });
        }

        Ok(res)
    }

    #[allow(dead_code)]
    pub fn read_space(&mut self) -> Result<(), InputError> {
        self.read()?;
        if self.buffer.is_empty() {
            return Err(InputError::UnexpectedEof {
                line: self.line,
                head: self.head,
            });
        }
        if self.buffer[self.head] != ' ' {
            return Err(InputError::UnexpectedCharacter {
                line: self.line,
                head: self.head,
            });
        }
        self.head += 1;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_eoln(&mut self) -> Result<(), InputError> {
        self.read()?;
        if self.buffer.is_empty() {
            return Err(InputError::UnexpectedEof {
                line: self.line,
                head: self.head,
            });
        }
        if self.buffer[self.head] != '\n' {
            return Err(InputError::UnexpectedCharacter {
                line: self.line,
                head: self.head,
            });
        }
        self.head += 1;

        Ok(())
    }

    pub fn read_eof(&mut self) -> Result<(), InputError> {
        self.read()?;
        if !self.buffer.is_empty() {
            return Err(InputError::UnexpectedCharacter {
                line: self.line,
                head: self.head,
            });
        }
        self.has_read_eof = true;

        Ok(())
    }
}

impl Default for InputChecker {
    fn default() -> Self {
        Self::new()
    }
}
