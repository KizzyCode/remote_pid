use crate::{ RemotePidError, RemotePid };
use std::{
	ptr, cell::RefCell, os::raw::c_char,
	ffi::{ CString, CStr }
};


/// Holds the last error
thread_local! {
	/// The last error _in the current thread_
	static LAST_ERR: RefCell<Option<CError>> = RefCell::new(None);
}
/// An `CError` that wraps the underlying error and a previous description
struct CError {
	code: u8,
	desc: CString
}
impl CError {
	/// Creates a new `CError` from
	fn new(error: RemotePidError) -> Self {
		// Convert the error to it's code
		let code = match error {
			RemotePidError::NonLocalConnection => 0x01,
			RemotePidError::Other(_) => 0xff
		};
		
		// Replace all `0`-bytes with the `�`-character
		let mut desc = Vec::new();
		error.to_string().bytes().for_each(|b| match b {
			0 => desc.extend_from_slice(b"\xEF\xBF\xBD"),
			b => desc.push(b)
		});
		
		Self{ code, desc: CString::new(desc).unwrap() }
	}
	
	/// Sets `error` as last error
	pub fn set(error: RemotePidError) {
		let error = Self::new(error);
		LAST_ERR.with(|l| l.replace(Some(error)));
	}
	/// Clears the last error
	pub fn clear() {
		LAST_ERR.with(|l| l.replace(None));
	}
}
/// Gets the code of the last error that occurred _in the current thread_
///
/// Possible codes are:
///  - `0x00`: No error occurred,
///  - `0x01`: One connection endpoint is not local
///  - `0xff`: Another error occurred
#[no_mangle]
pub extern "C" fn remote_pid_err_code() -> u8 {
	LAST_ERR.with(|e| {
		match e.borrow().as_ref().map(|e| e.code) {
			None => 0x00,
			Some(code) => code
		}
	})
}
/// Returns the description of the last error _in the current thread_ or `NULL` in case no error
/// occurred _in the current thread_
///
/// __Warning: The description is only valid until the next call of `remote_pid_str` or
/// `remote_pid_socked` _in the current thread___
#[no_mangle]
pub extern "C" fn remote_pid_err_desc() -> *const c_char {
	LAST_ERR.with(|err| {
		match err.borrow().as_ref().map(|e| &e.desc) {
			None => ptr::null(),
			Some(desc) => desc.as_ptr()
		}
	})
}


/// Gets the PID of the remote endpoint of the TCP-connection between `local` and `remote`
///
/// __Important: Call `remote_pid_err_code` to check if an error occurred__
#[no_mangle]
pub extern "C" fn remote_pid_str(local: *const c_char, remote: *const c_char) -> u32 {
	// Catch block
	let catch = || -> Result<u32, RemotePidError> {
		// Check the pointers
		assert!(!local.is_null(), "`local` is `NULL`");
		assert!(!remote.is_null(), "`remote` is `NULL`");
		
		// Cast the addresses and get the remote PID
		let local = unsafe{ CStr::from_ptr(local) }.to_str()?;
		let remote = unsafe{ CStr::from_ptr(remote) }.to_str()?;
		(local, remote).remote_pid()
	};
	
	// Clear the last error and call the catch block
	CError::clear();
	catch().unwrap_or_else(|error| {
		CError::set(error);
		0xff_ff_ff_ff
	})
}
/// Gets the PID of the remote endpoint of the TCP-connection between `local` and `remote`
///
/// __Important: Call `remote_pid_err_code` to check if an error occurred__
#[no_mangle]
pub extern "C" fn remote_pid_socket(fd: u64) -> u32 {
	// Clear the last error and call the implementation
	CError::clear();
	fd.remote_pid().unwrap_or_else(|error| {
		CError::set(error);
		0xff_ff_ff_ff
	})
}