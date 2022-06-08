use {
    sdl2::{video::WindowBuildError, IntegerOrSdlError},
    std::{convert::Infallible, fmt::Display},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum ErrorKind {
    Abstract,
    Window,
    Canvas,
}

impl From<&str> for ErrorKind {
    fn from(_: &str) -> Self {
        Self::Abstract
    }
}

impl From<String> for ErrorKind {
    fn from(_: String) -> Self {
        Self::Abstract
    }
}

impl From<WindowBuildError> for ErrorKind {
    fn from(_: WindowBuildError) -> Self {
        Self::Window
    }
}

impl From<IntegerOrSdlError> for ErrorKind {
    fn from(_: IntegerOrSdlError) -> Self {
        Self::Canvas
    }
}

impl From<Infallible> for ErrorKind {
    fn from(_: Infallible) -> Self {
        Self::Abstract
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}: {}", self.kind, self.error))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self {
            error: e.to_string(),
            kind: e.into(),
        }
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self {
            error: e.clone(),
            kind: e.into(),
        }
    }
}

impl From<WindowBuildError> for Error {
    fn from(e: WindowBuildError) -> Self {
        Self {
            error: e.to_string(),
            kind: e.into(),
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(e: IntegerOrSdlError) -> Self {
        Self {
            error: e.to_string(),
            kind: e.into(),
        }
    }
}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        Self {
            error: e.to_string(),
            kind: e.into(),
        }
    }
}
