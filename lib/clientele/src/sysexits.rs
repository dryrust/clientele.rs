// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

pub type SysexitsResult<T> = core::result::Result<T, SysexitsError>;

/// Standard `<sysexits.h>` preferable exit codes for programs on Unix systems.
///
/// Quoting the rationale given in the [`sysexits(3)`] man page on BSD systems:
///
/// > According to `style(9)`, it is not a good practice to call `exit(3)` with
/// > arbitrary values to indicate a failure condition when ending a program.
/// > Instead, the pre-defined exit codes from `sysexits` should be used, so
/// > the caller of the process can get a rough estimation about the failure
/// > class without looking up the source code.
///
/// [`sysexits(3)`]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/sysexits.3.html
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SysexitsError {
    #[default]
    /// Successful termination.
    EX_OK = 0,

    /// The command was used incorrectly, e.g., with the wrong number of
    /// arguments, a bad flag, a bad syntax in a parameter, or whatever.
    EX_USAGE = 64,

    /// The input data was incorrect in some way.
    ///
    /// This should only be used for user's data and not system files.
    EX_DATAERR = 65,

    /// An input file (not a system file) did not exist or was not readable.
    ///
    /// This could also include errors like "No message" to a mailer (if it
    /// cared to catch it).
    EX_NOINPUT = 66,

    /// The user specified did not exist.
    ///
    /// This might be used for mail addresses or remote logins.
    EX_NOUSER = 67,

    /// The host specified did not exist.
    ///
    /// This is used in mail addresses or network requests.
    EX_NOHOST = 68,

    /// A service is unavailable.
    ///
    /// This can occur if a support program or file does not exist.
    /// This can also be used as a catchall message when something you wanted
    /// to do doesn't work, but you don't know why.
    EX_UNAVAILABLE = 69,

    /// An internal software error has been detected.
    ///
    /// This should be limited to non-operating system related errors as
    /// possible.
    EX_SOFTWARE = 70,

    /// An operating system error has been detected.
    ///
    /// This is intended to be used for such things as "cannot fork", "cannot
    /// create pipe", or the like. It includes things like `getuid` returning a
    /// user that does not exist in the `/etc/passwd` file.
    EX_OSERR = 71,

    /// Some system file (e.g., `/etc/passwd`, `/var/run/utmp`, etc.) does not
    /// exist, cannot be opened, or has some sort of error (e.g., syntax
    /// error).
    EX_OSFILE = 72,

    /// A (user specified) output file cannot be created.
    EX_CANTCREAT = 73,

    /// An error occurred while doing I/O on some file.
    EX_IOERR = 74,

    /// Temporary failure, indicating something that is not really an error.
    ///
    /// In sendmail, this means that a mailer (e.g.) could not create a
    /// connection, and the request should be reattempted later.
    EX_TEMPFAIL = 75,

    /// The remote system returned something that was "not possible" during a
    /// protocol exchange.
    EX_PROTOCOL = 76,

    /// You did not have sufficient permission to perform the operation.
    ///
    /// This is not intended for file system problems, which should use
    /// `EX_NOINPUT` or `EX_CANTCREAT`, but rather for higher level
    /// permissions.
    EX_NOPERM = 77,

    /// Something was found in an unconfigured or misconfigured state.
    EX_CONFIG = 78,
}

impl SysexitsError {
    pub fn is_success(&self) -> bool {
        *self == Self::EX_OK
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn as_str(&self) -> &'static str {
        self.name()
    }

    #[cfg(feature = "std")]
    pub fn as_exit_code(&self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.as_u8())
    }

    #[cfg(feature = "std")]
    fn as_exit_status(&self) -> Option<std::process::ExitStatus> {
        match *self {
            Self::EX_OK => Some(std::process::ExitStatus::default()),
            _ => None,
        }
    }

    pub fn code(&self) -> Option<i32> {
        Some(self.as_i32())
    }

    pub fn name(&self) -> &'static str {
        use SysexitsError::*;
        match *self {
            EX_OK => "EX_OK",
            EX_USAGE => "EX_USAGE",
            EX_DATAERR => "EX_DATAERR",
            EX_NOINPUT => "EX_NOINPUT",
            EX_NOUSER => "EX_NOUSER",
            EX_NOHOST => "EX_NOHOST",
            EX_UNAVAILABLE => "EX_UNAVAILABLE",
            EX_SOFTWARE => "EX_SOFTWARE",
            EX_OSERR => "EX_OSERR",
            EX_OSFILE => "EX_OSFILE",
            EX_CANTCREAT => "EX_CANTCREAT",
            EX_IOERR => "EX_IOERR",
            EX_TEMPFAIL => "EX_TEMPFAIL",
            EX_PROTOCOL => "EX_PROTOCOL",
            EX_NOPERM => "EX_NOPERM",
            EX_CONFIG => "EX_CONFIG",
        }
    }

    pub fn summary(&self) -> &'static str {
        // See: https://github.com/openbsd/src/blob/master/include/sysexits.h
        use SysexitsError::*;
        match *self {
            EX_OK => "successful termination",
            EX_USAGE => "command line usage error",
            EX_DATAERR => "data format error",
            EX_NOINPUT => "cannot open input",
            EX_NOUSER => "addressee unknown",
            EX_NOHOST => "host name unknown",
            EX_UNAVAILABLE => "service unavailable",
            EX_SOFTWARE => "internal software error",
            EX_OSERR => "system error",
            EX_OSFILE => "critical OS file missing",
            EX_CANTCREAT => "can't create (user) output file",
            EX_IOERR => "input/output error",
            EX_TEMPFAIL => "temporary failure",
            EX_PROTOCOL => "remote error in protocol",
            EX_NOPERM => "permission denied",
            EX_CONFIG => "configuration error",
        }
    }
}

impl core::fmt::Display for SysexitsError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::str::FromStr for SysexitsError {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use SysexitsError::*;
        Ok(match input {
            "EX_OK" => EX_OK,
            "EX_USAGE" => EX_USAGE,
            "EX_DATAERR" => EX_DATAERR,
            "EX_NOINPUT" => EX_NOINPUT,
            "EX_NOUSER" => EX_NOUSER,
            "EX_NOHOST" => EX_NOHOST,
            "EX_UNAVAILABLE" => EX_UNAVAILABLE,
            "EX_SOFTWARE" => EX_SOFTWARE,
            "EX_OSERR" => EX_OSERR,
            "EX_OSFILE" => EX_OSFILE,
            "EX_CANTCREAT" => EX_CANTCREAT,
            "EX_IOERR" => EX_IOERR,
            "EX_TEMPFAIL" => EX_TEMPFAIL,
            "EX_PROTOCOL" => EX_PROTOCOL,
            "EX_NOPERM" => EX_NOPERM,
            "EX_CONFIG" => EX_CONFIG,
            _ => match u8::from_str(input) {
                Ok(code) => return Self::try_from(code).map_err(|_| ()),
                Err(_) => return Err(()),
            },
        })
    }
}

impl TryFrom<u8> for SysexitsError {
    type Error = u8;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        use SysexitsError::*;
        Ok(match code {
            0 => EX_OK,
            64 => EX_USAGE,
            65 => EX_DATAERR,
            66 => EX_NOINPUT,
            67 => EX_NOUSER,
            68 => EX_NOHOST,
            69 => EX_UNAVAILABLE,
            70 => EX_SOFTWARE,
            71 => EX_OSERR,
            72 => EX_OSFILE,
            73 => EX_CANTCREAT,
            74 => EX_IOERR,
            75 => EX_TEMPFAIL,
            76 => EX_PROTOCOL,
            77 => EX_NOPERM,
            78 => EX_CONFIG,
            _ => return Err(code),
        })
    }
}

impl TryFrom<i32> for SysexitsError {
    type Error = i32;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        Self::try_from(code as u8).map_err(|_| code)
    }
}

#[cfg(feature = "std")]
impl TryFrom<std::process::ExitStatus> for SysexitsError {
    type Error = Option<i32>;

    fn try_from(status: std::process::ExitStatus) -> Result<Self, Self::Error> {
        match status.code() {
            Some(code) => Self::try_from(code).map_err(|_| Some(code)),
            None => Err(None),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::boxed::Box<dyn std::error::Error>> for SysexitsError {
    fn from(error: std::boxed::Box<dyn std::error::Error>) -> Self {
        Self::EX_SOFTWARE
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for SysexitsError {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match error.kind() {
            AddrInUse => Self::EX_TEMPFAIL,
            AddrNotAvailable => Self::EX_USAGE,
            AlreadyExists => Self::EX_CANTCREAT,
            BrokenPipe => Self::EX_IOERR,
            ConnectionAborted => Self::EX_PROTOCOL,
            ConnectionRefused => Self::EX_UNAVAILABLE,
            ConnectionReset => Self::EX_PROTOCOL,
            Interrupted => Self::EX_TEMPFAIL,
            InvalidData => Self::EX_DATAERR,
            InvalidInput => Self::EX_DATAERR,
            NotConnected => Self::EX_PROTOCOL,
            NotFound => Self::EX_NOINPUT,
            Other => Self::EX_UNAVAILABLE,
            OutOfMemory => Self::EX_TEMPFAIL,
            PermissionDenied => Self::EX_NOPERM,
            TimedOut => Self::EX_IOERR,
            UnexpectedEof => Self::EX_IOERR,
            Unsupported => Self::EX_SOFTWARE,
            WouldBlock => Self::EX_IOERR,
            WriteZero => Self::EX_IOERR,
            _ => Self::EX_UNAVAILABLE, // catch-all
        }
    }
}

#[cfg(feature = "gofer")]
impl From<gofer::Error> for SysexitsError {
    fn from(error: gofer::Error) -> Self {
        // See: https://docs.rs/gofer/latest/gofer/enum.Error.html
        use gofer::Error::*;
        match error {
            InvalidUrl(_) => Self::EX_DATAERR,
            UnknownScheme(_) => Self::EX_DATAERR,
            _ => Self::EX_SOFTWARE, // catch-all
        }
    }
}

#[cfg(feature = "serde-json")]
impl From<serde_json::Error> for SysexitsError {
    fn from(error: serde_json::Error) -> Self {
        // See: https://docs.rs/serde_json/latest/serde_json/struct.Error.html
        use serde_json::error::Category::*;
        match error.classify() {
            Io => Self::EX_IOERR,
            Syntax => Self::EX_DATAERR,
            Data => Self::EX_DATAERR,
            Eof => Self::EX_NOINPUT,
            _ => Self::EX_SOFTWARE, // catch-all
        }
    }
}

#[cfg(feature = "tokio")]
impl From<tokio::task::JoinError> for SysexitsError {
    fn from(error: tokio::task::JoinError) -> Self {
        // See: https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html
        Self::EX_SOFTWARE
    }
}

#[cfg(feature = "std")]
impl std::process::Termination for SysexitsError {
    fn report(self) -> std::process::ExitCode {
        (self as u8).into()
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SysexitsError {}

#[cfg(feature = "error-stack")]
impl error_stack::Context for SysexitsError {}

/// Exit the process with the given exit code.
#[cfg(feature = "std")]
pub fn exit(code: SysexitsError) -> ! {
    std::process::exit(code as i32);
}

/// Exit the process with the given exit code and error string to be printed to
/// standard error.
#[cfg(feature = "std")]
macro_rules! abort {
    ($code:expr, $($t:tt)*) => {{
        std::eprintln!($($t)*);
        exit($code)
    }};
}
