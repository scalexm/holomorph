# protocol_derive

Procedural macro enabling auto-derive implementations of the `Encode` and
`Decode` traits from the `protocol` crate. Example:

```rust
use protocol_derive::{Encode, Decode};
use std::borrow::Cow;

#[derive(Clone, Encode, Decode)]
#[protocol(id = 47)]
struct HelloMessage<'a> {
    id: u32,
    #[protocol(var)] key: i64,
    #[protocol(flag)] has_foo: bool,
    #[protocol(flag)] has_bar: bool,
    name: &'a str,
    #[protocol(var_length)] actors: Cow<'a, [Actor<'a>]>,
}

#[derive(Copy, Clone, Encode, Decode)]
#[protocol(id = 52)]
struct Actor<'a> {
    level: i32,
    name: &'a str,
    #[protocol(var)] emotes: &'a [u8],
}
```

Usable attributes:
* `#[protocol(crate = "protocol")]`: give a path where to find the `protocol`
  crate; will default to `crate` if absent
* `#[protocol(id = 47)]`: generate a `Encode::id` / `Decode::id` method
   implementation returning `47`
* `#[protocol(var)]`: dynamically encode an integer type on a variable number
   of bytes *OR* dynamically encode the length of an `&'_ [u8]` on a variable
   number of bytes
* `#[protocol(flag)]`: treat a `bool` as a bit offset of an `u8`; consecutive
  `bool` fields marked with `#[protocol(flag)]` will use the same `u8` (up to
  `8` consecutive fields of course)
* `#[protocol(var_length)]`: dynamically encode the length of an array of
   arbitrary types on a variable number of bytes
* `#[protocol(var_content)]`: dynamically encode each value of an array of
  integer types on a variable number of bytes; this attribute can be combined
  with the previous one so that both the length and the contents are
  dynamically encoded
