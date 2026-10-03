# Credits

## Historical sysexits implementation

The former `lib/clientele/src/sysexits.rs` implementation used definitions from
the [`sysexits.h`] header found on BSD operating systems. Its [BSD-3-Clause]
notice is retained below as historical attribution.

Since Clientele 0.3.4, the implementation comes from [known-errors]'s
`known_errors::sysexits` module. Clientele exposes `SysexitsError` and
`SysexitsResult` at the crate root, and `exit` with the `std` feature, through
the re-exports in [`lib/clientele/src/lib.rs`](lib/clientele/src/lib.rs).
There is no current `clientele::sysexits` module or in-tree `sysexits.rs` file.

```
/*
 * Copyright (c) 1987 Regents of the University of California.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)sysexits.h	4.8 (Berkeley) 4/3/91
 */
 ```

[`sysexits.h`]: https://github.com/openbsd/src/blob/master/include/sysexits.h
[BSD-3-Clause]: https://spdx.org/licenses/BSD-3-Clause.html
[known-errors]: https://docs.rs/known-errors/latest/known_errors/sysexits/
