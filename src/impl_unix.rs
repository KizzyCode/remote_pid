use crate::RemotePidError;
use std::{
	io, error::Error, path::Path, str::FromStr,
	fmt::{ self, Display, Formatter }, process::{ self, Command },
	net::{ ToSocketAddrs, TcpStream, SocketAddr }
};


/// Trusted `lsof` paths
const LSOF: [&str; 2] = ["/usr/sbin/lsof", "/usr/bin/lsof"];


/// An `lsof` related error
#[derive(Debug)]
enum LsofError {
	BinaryNotFound,
	NonZeroExitCode{ exit_code: Option<i32> },
	InvalidPid,
	UnknownRemotePid,
	InvalidMultipleRemotePids
}
impl Display for LsofError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl Error for LsofError {}
impl From<LsofError> for RemotePidError {
	fn from(lsof_error: LsofError) -> Self {
		RemotePidError::Other(Box::new(lsof_error))
	}
}


/// Gets the PID of the process at the remote endpoint of `connection`
pub fn tcp_stream(connection: &TcpStream) -> Result<u32, RemotePidError> {
	addr_pair(connection.local_addr()?, connection.peer_addr()?)
}
/// Gets the PID of the remote endpoint of the TCP-connection between `local` and `remote`
pub fn addr_pair(local: impl ToSocketAddrs, remote: impl ToSocketAddrs)
	-> Result<u32, RemotePidError>
{
	// Get addresses
	let local: SocketAddr = local.to_socket_addrs()?.next().ok_or(io::ErrorKind::InvalidInput)?;
	let remote: SocketAddr = remote.to_socket_addrs()?.next().ok_or(io::ErrorKind::InvalidInput)?;
	
	// Validate that the connection runs over localhost
	if !local.ip().is_loopback() || !remote.ip().is_loopback() {
		Err(RemotePidError::NonLocalConnection)?
	}
	
	// Run `lsof` to get the PID
	lsof(local, remote)
}
/// Gets the PID at the remote endpoint of the TCP connection referenced by `fd`
#[cfg(feature = "c_api")]
pub fn raw_fd(fd: u64) -> Result<u32, RemotePidError> {
	use std::os::unix::io::{ RawFd, FromRawFd };
	
	// Validate and convert the raw FD
	let fd = match fd {
		fd if fd > RawFd::max_value() as u64 => Err(io::ErrorKind::InvalidInput)?,
		fd => fd as RawFd
	};
	
	// Create the stream from the raw FD
	let stream: TcpStream = unsafe{ TcpStream::from_raw_fd(fd) };
	tcp_stream(&stream)
}


/// Calls `lsof` to get the remote PID between `local` and `remote`
fn lsof(local: SocketAddr, remote: SocketAddr) -> Result<u32, RemotePidError> {
	// Find lsof
	let lsof: &str = LSOF.iter().find_map(|p| match Path::new(p).is_file() {
		true => Some(p),
		false => None
	}).ok_or(LsofError::BinaryNotFound)?;
	
	// Prepare interface-strings
	let local = format!("TCP@{}", local);
	let remote = format!("TCP@{}", remote);
	
	// Run lsof
	let output = Command::new(lsof)
		.arg("-Fp").arg("-i").arg(local).arg("-i").arg(remote).output()?;
	let output = match output.status.code() {
		Some(0) => String::from_utf8(output.stdout)?,
		exit_code => Err(LsofError::NonZeroExitCode{ exit_code })?
	};
	
	// Extract PIDs
	let (own_pid, mut pids) = (process::id(), Vec::new());
	for line in output.lines().filter(|l| l.starts_with('p')) {
		match u32::from_str(line.split_at(1).1) {
			Ok(pid) if pid == own_pid => (),
			Ok(pid) => pids.push(pid),
			Err(_) => Err(LsofError::InvalidPid)?
		}
	}
	
	// Check result
	match pids.len() {
		1 => Ok(pids[0]),
		0 => Err(LsofError::UnknownRemotePid)?,
		_ => Err(LsofError::InvalidMultipleRemotePids)?
	}
}