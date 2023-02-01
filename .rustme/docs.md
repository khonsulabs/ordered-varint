Provides variable-length signed and unsigned integer encoding that is
byte-orderable.

[![crate version](https://img.shields.io/crates/v/ordered-varint.svg)](https://crates.io/crates/ordered-varint)
[![Live Build Status](https://img.shields.io/github/actions/workflow/status/khonsulabs/ordered-varint/tests.yml?branch=main)](https://github.com/khonsulabs/ordered-varint/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/ordered-varint/coverage/badge.svg)](https://khonsulabs.github.io/ordered-varint/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/ordered-varint/main/ordered_varint/)

This crate provides the `Variable` trait which encodes and decodes integers to
an abbreviated format that ranges from 1 to 16 bytes. All signed integer types
(i8, i16, i32, i64, and i128) are comparable with each other, and all unsigned
integer types (u8, u16, u32, u64, and u128) are comparable with each other.
**However, encoded signed and unsigned values are not able to be meaningfully
compared to one another.**

## Example

To run the included example, execute `cargo run --example demo`. The output will
look similar to this:

```text
Original bytes: 1968
Encoded bytes: 1068
1 encodes as [01]
3 encodes as [03]
7 encodes as [07]
15 encodes as [0f]
31 encodes as [10, 1f]
63 encodes as [10, 3f]
127 encodes as [10, 7f]
255 encodes as [10, ff]
511 encodes as [11, ff]
[...]
1329227995784915872903807060280344575 encodes as [f0, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
2658455991569831745807614120560689151 encodes as [f1, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
5316911983139663491615228241121378303 encodes as [f3, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
10633823966279326983230456482242756607 encodes as [f7, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
```

Another example showing signed encoding is available by running `cargo run
--example demo-signed`. The output will look similar to this:

```text
Original bytes: 3920
Encoded bytes: 2165
-5316911983139663491615228241121378305 encodes as [03, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
-2658455991569831745807614120560689153 encodes as [05, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
-1329227995784915872903807060280344577 encodes as [06, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
-664613997892457936451903530140172289 encodes as [07, 7f, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
[...]
-65 encodes as [77, bf]
-33 encodes as [77, df]
-17 encodes as [77, ef]
-9 encodes as [77, f7]
-5 encodes as [7b]
-3 encodes as [7d]
1 encodes as [81]
3 encodes as [83]
7 encodes as [87]
15 encodes as [88, 0f]
31 encodes as [88, 1f]
63 encodes as [88, 3f]
[...]
1329227995784915872903807060280344575 encodes as [f8, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
2658455991569831745807614120560689151 encodes as [f9, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
5316911983139663491615228241121378303 encodes as [fb, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
10633823966279326983230456482242756607 encodes as [ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff]
```
