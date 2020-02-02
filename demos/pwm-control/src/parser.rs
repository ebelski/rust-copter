//! Command parser

use crate::bsp::usb;

use core::num;
use core::str;

/// A generalized read trait, letting us switch our command input
/// source from USB to serial, if desired
pub trait Read {
    fn read(&mut self, buffer: &mut [u8]) -> usize;
}

impl Read for usb::Reader {
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        usb::Reader::read(self, buffer)
    }
}

/// The four available PWM outputs
#[derive(Debug)]
pub enum Output {
    A,
    B,
    C,
    D,
}

/// A user command
#[derive(Debug)]
pub enum Command {
    /// Set the duty cycle of the provided output to the
    /// specified percent.
    ///
    /// `percent` is bound from the closed range `[0, 100]`
    SetDuty { output: Output, percent: f64 },
    /// Read all duty cycle values
    ReadDuty,
}

/// Things that could go wrong when parsing commands
#[derive(Debug)]
pub enum ParserError {
    /// Invalid command prefix (the thing before the '.')
    InvalidPrefix(char),
    /// Need to have a dot '.' after the prefix
    InvalidSeparator(char),
    /// Not a number
    InvalidNumber(num::ParseFloatError),
    /// String parsing error
    Invalid(str::Utf8Error),
}

impl From<str::Utf8Error> for ParserError {
    fn from(err: str::Utf8Error) -> ParserError {
        ParserError::Invalid(err)
    }
}

impl From<num::ParseFloatError> for ParserError {
    fn from(err: num::ParseFloatError) -> Self {
        ParserError::InvalidNumber(err)
    }
}

/// Internal parser buffer size
const PARSER_BUFFER_LEN: usize = 256;

/// A command parser interprets commands from an underlying
/// `Read` type
pub struct Parser<R> {
    reader: R,
    buffer: [u8; PARSER_BUFFER_LEN],
    bytes_read: usize,
}

impl<R: Read> Parser<R> {
    /// Create a new reader that parses commands from the provided reader
    pub fn new(reader: R) -> Self {
        Parser {
            reader,
            buffer: [0; PARSER_BUFFER_LEN],
            bytes_read: 0,
        }
    }

    /// Parse a command. Returns `Ok(None)` if data is not available. Returns
    /// `Ok(Some(Command))` when we've parsed a valid command, or `Err(ParserError)`
    /// if we've found something wrong in the buffer.
    pub fn parse(&mut self) -> Result<Option<Command>, ParserError> {
        let bytes_read = self.reader.read(&mut self.buffer[self.bytes_read..]);
        self.bytes_read += bytes_read;
        match parse(&self.buffer[..self.bytes_read]) {
            Ok(None) => Ok(None),
            Ok(Some(command)) => {
                self.bytes_read = 0;
                Ok(Some(command))
            }
            Err(err) => {
                self.bytes_read = 0;
                Err(err)
            }
        }
    }
}

fn parse(buffer: &[u8]) -> Result<Option<Command>, ParserError> {
    // Match a valid output immediately
    let output = if let Some(output) = buffer.get(0) {
        match *output {
            b'A' => Output::A,
            b'B' => Output::B,
            b'C' => Output::C,
            b'D' => Output::D,
            b'r' => return Ok(Some(Command::ReadDuty)),
            _ => return Err(ParserError::InvalidPrefix(*output as char)),
        }
    } else {
        return Ok(None);
    };

    // We have a valid output! Find the separator...
    if let Some(separator) = buffer.get(1) {
        match *separator {
            b'.' => (),
            _ => return Err(ParserError::InvalidSeparator(*separator as char)),
        }
    } else {
        return Ok(None);
    }

    // Now, we wait for the end of the string...
    match buffer.last() {
        Some(b'\r') => (),
        _ => return Ok(None),
    }

    let pct_str = str::from_utf8(&buffer[2..])?;
    if pct_str.is_empty() {
        return Ok(None);
    }

    let pct: f64 = str::parse(pct_str.trim())?;
    // TODO figure out where fmax and fmin are...
    let percent = if pct < 0.0f64 {
        0.0f64
    } else if pct > 100.0f64 {
        100.0f64
    } else {
        pct
    };
    Ok(Some(Command::SetDuty { output, percent }))
}
