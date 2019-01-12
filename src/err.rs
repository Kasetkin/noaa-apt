use std::error::Error as StdError;

use hound;
use png;
use log;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(std::io::Error), // Input/output
    WavOpen(String), // About WAV decoding/opening
    PngWrite(String), // About PNG encoding/writing
    Internal(String), // noaa-apt internal errors
    FeatureNotAvailable(Vec<String>), // Functionality not available because the
                                      // program was compiled without those features
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::WavOpen(ref msg) => f.write_str(msg.as_str()),
            Error::PngWrite(ref msg) => f.write_str(msg.as_str()),
            Error::Internal(ref msg) => f.write_str(msg.as_str()),
            Error::FeatureNotAvailable(ref features) =>
                write!(f, "Program compiled without support for features: {:?}",
                    features),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<hound::Error> for Error {
    fn from(err: hound::Error) -> Error {
        match err {
            hound::Error::IoError(io_error) => Error::Io(io_error),
            hound::Error::FormatError(_) => Error::WavOpen(err.to_string()),
            hound::Error::TooWide => Error::Internal(err.to_string()),
            hound::Error::UnfinishedSample => Error::Internal(err.to_string()),
            hound::Error::Unsupported => Error::WavOpen(err.to_string()),
            hound::Error::InvalidSampleFormat => Error::Internal(err.to_string()),
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Error {
        Error::Internal(err.description().to_string())
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Error {
        match err {
            png::EncodingError::IoError(io_error) => Error::Io(io_error),
            png::EncodingError::Format(_) => Error::PngWrite(err.description().to_string()),
        }
    }
}
