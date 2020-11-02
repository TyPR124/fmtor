use std::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result, UpperExp,
    UpperHex,
};

use crate::FmtOr;

struct Baz;
impl Display for Baz {
    fn fmt(&self, out: &mut Formatter) -> Result {
        Display::fmt("Baz", out)
    }
}

macro_rules! tests {
    ($(
        $test_name:ident: $Trait:ident ($fmt_str:expr, $t:ident $(=> $deref:expr)?) { $(
            $value:expr $( => $expected:expr )?
        ),*$(,)? }
    )*) => { $(
        #[test]
        fn $test_name() {
            fn test_inner<T: $Trait>(opt: &Option<T>, expected: Option<&'static str>) {
                if let Some($t) = opt {
                    #[allow(unused_variables)]
                    let want = format!($fmt_str, $t);
                    $(let want = format!($fmt_str, $deref);)?
                    let got = format!($fmt_str, opt.fmt_or(""));
                    assert_eq!(want, got);
                    if let Some(expected) = expected {
                        assert_eq!(expected, got)
                    }
                } else {
                    assert_eq!("", format!($fmt_str, opt.fmt_or_empty()));
                    assert_eq!("foo", format!($fmt_str, opt.fmt_or("foo")));
                    assert_eq!("bar", format!($fmt_str, opt.fmt_or_else(||"bar")));
                    assert_eq!("Baz", format!($fmt_str, opt.fmt_or(Baz)));
                    assert_eq!("Baz", format!($fmt_str, opt.fmt_or_else(|| Baz)));
                }
            }

            $(
                let mut report = format!("Test case: [{}] ({}, {})", stringify!($Trait), stringify!($fmt_str), stringify!($value));
                #[allow(unused_variables)]
                let expected = Option::<&'static str>::None;
                $(let expected = Some($expected);)?
                if let Some(expected) = expected {
                    report += " == ";
                    report += expected;
                }
                println!("{}", report);
                for case in &[Some($value), None] {
                    test_inner(case, expected);
                }
            )*
        }
    )*};
}

tests!(
    test_display: Display("{}", t) {
        7 => "7",
        "X".to_string() => "X",
    }
    test_debug: Debug("{:?}", t) {
        7 => "7",
        "x" => "\"x\"",
        Some(7) => "Some(7)"
    }
    test_debug_alt: Debug("{:#?}", t) {
        Some(7) => "Some(\n    7,\n)"
    }
    test_binary: Binary("{:b}", t) {
        7 => "111",
    }
    test_binary_alt: Binary("{:#b}", t) {
        7 => "0b111",
    }
    test_octal: Octal("{:o}", t) {
        42 => "52"
    }
    test_octal_alt: Octal("{:#o}", t) {
        42 => "0o52"
    }
    test_upperhex: UpperHex("{:X}", t) {
        10 => "A"
    }
    test_upperhex_alt: UpperHex("{:#X}", t) {
        10 => "0xA"
    }
    test_lowerhex: LowerHex("{:x}", t) {
        10 => "a"
    }
    test_lowerhex_alt: LowerHex("{:#x}", t) {
        10 => "0xa"
    }
    test_upperexp: UpperExp("{:E}", t) {
        10 => "1E1"
    }
    test_lowerexp: LowerExp("{:e}", t) {
        10 => "1e1"
    }
    test_pointer: Pointer("{:p}", t => *t) {
        Box::new(10),
        &10,
        &10 as *const _,
        Box::new(Baz) => "0x1",
        &(),
        &mut (),
        std::sync::Arc::new(()),
        0x42 as *const Baz => "0x42",
        0x42 as *mut Baz => "0x42",
    }
);
