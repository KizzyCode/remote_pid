//! This crate allows you to get the PID of the remote endpoint of a TCP connection if the remote
//! endpoint is running on the same host.
//!
//! # Example:
//! ```ignore
//! use remote_pid::RemotePid;
//!
//! let connection: TcpStream = /* my TCP connection */;
//!
//! let pid = connection.remote_pid().expect("Failed to get the remote PID");
//!
//! println!("The remote PID is {}", pid);
//! ```


// Include platform dependent implementation
#[cfg(target_family = "unix")] #[path="impl_unix.rs"]
mod get_pid;

#[cfg(not(any(target_family = "unix")))]
compile_error!("The current platform is unsupported");


// Include the C API if featured
#[cfg(feature = "c_api")]
pub mod c_api;


// Includes
use std::{
	io, error::Error, str::Utf8Error, string::FromUtf8Error,
	net::{ ToSocketAddrs, TcpStream }, fmt::{ self, Display, Formatter }
};


/// Defines an interface to get the PID of a remote connection endpoint
pub trait RemotePid {
	/// The PID of the remote endpoint
	///
	/// _Warning: This method will fail if both endpoints have the same PID_
	fn remote_pid(&self) -> Result<u32, RemotePidError>;
}
impl RemotePid for TcpStream {
	/// Gets the PID of the process at the remote endpoint of the TCP connection
	///
	/// _Warning: This method will fail if both endpoints have the same PID_
	fn remote_pid(&self) -> Result<u32, RemotePidError> {
		get_pid::tcp_stream(self)
	}
}
impl<T: ToSocketAddrs, U: ToSocketAddrs> RemotePid for (T, U) {
	/// Gets the PID of the remote endpoint of the TCP connection between the addresses `T` and `U`
	///
	/// _Warning: This method will fail if both endpoints have the same PID_
	fn remote_pid(&self) -> Result<u32, RemotePidError> {
		get_pid::addr_pair(&self.0, &self.1)
	}
}
#[cfg(feature = "c_api")]
impl RemotePid for u64 {
	/// Gets the PID at the remote endpoint of the TCP connection referenced by the raw FD
	///
	/// _Warning: This method will fail if both endpoints have the same PID_
	fn remote_pid(&self) -> Result<u32, RemotePidError> {
		get_pid::raw_fd(*self)
	}
}


/// A `remote_pid`-related error
#[derive(Debug)]
pub enum RemotePidError {
	/// The connection has a non-local endpoint
	NonLocalConnection,
	/// Another (usually implementation specific) error
	Other(Box<dyn Error + 'static>)
}
impl Display for RemotePidError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl Error for RemotePidError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RemotePidError::Other(source) => Some(source.as_ref()),
			_ => None
		}
	}
}
impl From<io::Error> for RemotePidError {
	fn from(io_error: io::Error) -> Self {
		RemotePidError::Other(Box::new(io_error))
	}
}
impl From<io::ErrorKind> for RemotePidError {
	fn from(io_error: io::ErrorKind) -> Self {
		RemotePidError::Other(Box::new(io::Error::from(io_error)))
	}
}
impl From<Utf8Error> for RemotePidError {
	fn from(utf8_error: Utf8Error) -> Self {
		RemotePidError::Other(Box::new(utf8_error))
	}
}
impl From<FromUtf8Error> for RemotePidError {
	fn from(utf8_error: FromUtf8Error) -> Self {
		RemotePidError::Other(Box::new(utf8_error))
	}
}