[![BSD 2-Clause License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![MIT License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/MIT)

# remote_pid
This crate allows you to get the PID of the remote endpoint of a TCP connection if the remote
endpoint is running on the same host. 


## FAQ:
 - What is the purpose of this crate? This crate can be used in IPC contexts to get the PID of the
   remote caller and authenticate it subsequently.
 - Why TCP? TCP on localhost is the only connection-based IPC mechanism that works on nearly every
   desktop OS and the only IPC mechanism that can be used to authenticate the other endpoint on
   nearly every desktop OS.


## Caveats:
 - Currently Unix only
 - Usually needs root permissions to identify endpoints owned by a different user
 - Relies on an external program (`lsof` under Unix)


# C-API
To build and install the library with an `extern "C"`-interface, go into `c_api` and run
`make install`. The files are placed into `$PREFIX/lib` and `$PREFIX/include` where `$PREFIX`
defaults to `/usr/local`.