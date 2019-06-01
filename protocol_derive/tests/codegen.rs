use protocol::{Decode, Encode};
use protocol_derive::{Decode, Encode};
use std::borrow::Cow;

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
#[protocol(crate = "protocol", id = 1)]
struct Foo;

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
#[protocol(crate = "protocol", id = 2)]
struct Bar<'a> {
    a: u8,
    b: i16,
    #[protocol(var)]
    c: u32,
    d: f64,
    e: &'a str,
    f: [u32; 5],
    #[protocol(var)]
    g: &'a [u8],
}

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
#[protocol(crate = "protocol", id = 3)]
struct Baz<'a> {
    a: Cow<'a, [Bar<'a>]>,
    #[protocol(var_length)]
    b: Cow<'a, [Foo]>,
    #[protocol(flag)]
    c: bool,
    #[protocol(flag)]
    d: bool,
    e: f32,
    #[protocol(flag)]
    f: bool,
    #[protocol(var_contents)]
    g: Cow<'a, [i32]>,
    #[protocol(var_length, var_contents)]
    h: Cow<'a, [i64]>,
    #[protocol(flag)]
    i: bool,
}

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
#[protocol(crate = "protocol", id = 4)]
enum FooBaz<'a> {
    Foo(Foo),
    Baz(Baz<'a>),
}

#[test]
fn test_codegen() {
    let mut buf = Vec::new();

    let g = &[1, 2, 3];
    let bar1 = Bar {
        a: 5,
        b: 655,
        c: 12354434,
        d: 2.71828,
        e: "foo",
        f: [1, 2, 3, 4, 5],
        g: &g[..],
    };

    let bar2 = Bar {
        a: 24,
        b: -21022,
        c: 6455634,
        d: -2.71828,
        e: "bar",
        f: [4535, 2234, 4, 5, 6],
        g: &g[..],
    };

    let a = &[bar1, bar2];
    let b = &[Foo, Foo, Foo];
    let baz = Baz {
        a: a[..].into(),
        b: b[..].into(),
        c: true,
        d: false,
        e: 4.5354,
        f: true,
        g: vec![1, 2, 3].into(),
        h: vec![1, 3, 4, 5].into(),
        i: true,
    };

    baz.encode(&mut buf);

    let foobaz1 = FooBaz::Foo(Foo);
    foobaz1.encode(&mut buf);

    let foobaz2 = FooBaz::Baz(baz.clone());
    foobaz2.encode(&mut buf);

    let mut buf = &buf[0..];
    assert_eq!(Baz::decode(&mut buf), Ok(baz));
    assert_eq!(FooBaz::decode(&mut buf), Ok(foobaz1));
    assert_eq!(FooBaz::decode(&mut buf), Ok(foobaz2));
}
