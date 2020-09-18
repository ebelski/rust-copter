//! Command parser

use crate::bsp::usb;

use core::num;
use core::str;

use esc::QuadMotor;

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

/// A user command
#[derive(Debug)]
pub enum Command {
    /// Set the throttle of the provided output to the
    /// specified percent.
    ///
    /// `percent` is bound from the closed range `[0, 100]`
    SetThrottle { output: QuadMotor, percent: f32 },
    /// Read system settings
    ReadSettings,
    /// Reset all throttle values
    ResetThrottle,
    /// Kill switch
    ///
    /// Pressing the kill switch sets all duty cycles to 0%, so there
    /// is no output signal. The software will stop responding, and it
    /// will require a reset to fix.
    KillSwitch,
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
    /// Invalid percentage (value must be bound by [0, 100])
    InvalidPercentage(f32),
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
            b'A' => QuadMotor::A,
            b'B' => QuadMotor::B,
            b'C' => QuadMotor::C,
            b'D' => QuadMotor::D,
            b'r' => return Ok(Some(Command::ReadSettings)),
            b' ' => return Ok(Some(Command::ResetThrottle)),
            b'\\' => return Ok(Some(Command::KillSwitch)),
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

    let percent: f32 = str::parse(pct_str.trim())?;
    if 0.0f32 <= percent && percent <= 100.0f32 {
        Ok(Some(Command::SetThrottle { output, percent }))
    } else {
        Err(ParserError::InvalidPercentage(percent))
    }
}
