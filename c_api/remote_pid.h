// The "remote_pid" library (including this header file) is dual licensed under either the MIT or
// BSD-2-clause license
//
//
// MIT License
//
// Copyright (c) 2018 Keziah Biermann
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
//
// BSD-2-Clause License
//
// Copyright (c) 2018, Keziah Biermann
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


#ifndef REMOTE_PID_REMOTE_PID_H
#define REMOTE_PID_REMOTE_PID_H

#include <stdint.h>


/// Gets the code of the last error that occurred _in the current thread_
///
/// \return `0x00` if no error occurred, `0x01` if one connection endpoint is not local or `0xff` if
///         another error occurred
uint8_t remote_pid_err_code();

/// Returns the description of the last error _in the current thread_ or `NULL` in case no error
/// occurred _in the current thread_
///
/// \return The description as UTF-8 string or `NULL` in case no error occurred
char const* remote_pid_err_desc();


/// Gets the PID of the remote endpoint of the TCP-connection between `local` and `remote`
///
/// \warning Call `remote_pid_err_code` to check if an error occurred!
///
/// \param local The local endpoint as string
/// \param remote The remote endpoint as string
/// \return The PID of the remote endpoint
uint32_t remote_pid_str(char const* local, char const* remote);

/// Gets the PID of the remote endpoint of the TCP-connection between `local` and `remote`
///
/// \param fd The file descriptor of the TCP connection
/// \return The PID of the remote endpoint
uint32_t remote_pid_socket(uint64_t fd);


#endif //REMOTE_PID_REMOTE_PID_H
