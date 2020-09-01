#![forbid(unsafe_code)]

#[macro_use]
extern crate with_locals;

use ::core::fmt::Display;

#[test]
#[with]
fn hex ()
{
    #[with]
    fn hex (n: u32) -> &'self dyn Display
    {
        &format_args!("{:#x}", n)
    }

    #[with]
    let s_hex = hex(66);
    assert_eq!(s_hex.to_string(), "0x42");
}

mod to_str {
    trait ToStr {
        #[with('local)]
        fn to_str (self: &'_ Self) -> &'local str
        ;
    }

    impl ToStr for u32 {
        #[with('local)]
        fn to_str (self: &'_ u32) -> &'local str
        {
            let mut x = *self;
            if x == 0 { return "0"; }
            let mut buf = [b' '; 1 + 3 + 3 + 3]; // u32::MAX ~ 4_000_000_000
            let mut cursor = &mut buf[..];
            while x > 0 {
                let (last, cursor_) = cursor.split_last_mut().unwrap();
                cursor = cursor_;
                *last = b'0' + (x % 10) as u8;
                x /= 10;
            }
            let len = cursor.len();
            ::core::str::from_utf8(&buf[len ..])
                .unwrap()
        }
    }

    #[test]
    #[with]
    fn basic ()
    {

        #[with]
        let n: &str = ::core::u32::MAX.to_str();
        dbg!(n);
        assert_eq!(n.parse(), Ok(::core::u32::MAX));
    }

    #[test]
    #[with]
    fn romans ()
    {

        struct Roman(u8); impl ToStr for Roman {
            #[with]
            fn to_str (self: &'_ Self) -> &'self str
            {
                let mut buf = [b' '; 1 + 4 + 4];  // C LXXX VIII or CC XXX VIII
                let mut start = buf.len();
                let mut prepend = |b| {
                    start = start.checked_sub(1).unwrap();
                    buf[start] = b;
                };
                let mut n = self.0;
                if n == 0 {
                    panic!("Vade retro!");
                }
                const DIGITS: [u8; 7] = *b"IVXLCDM";
                (0 ..= 2).for_each(|shift| {
                    #[allow(nonstandard_style)]
                    let (I, V, X) = (
                        DIGITS[2 * shift],
                        DIGITS[2 * shift + 1],
                        DIGITS[2 * shift + 2],
                    );
                    match n % 10 {
                        | units @ 0 ..= 3 => {
                            (0 .. units).for_each(|_| prepend(I));
                        },
                        | 4 => {
                            prepend(V);
                            prepend(I);
                        },
                        | units @ 5 ..= 8 => {
                            (0 .. units - 5).for_each(|_| prepend(I));
                            prepend(V);
                        },
                        | 9 => {
                            prepend(X);
                            prepend(I);
                        },
                        | _ => unreachable!(),
                    }
                    n /= 10;
                });
                return ::core::str::from_utf8(&buf[start ..]).unwrap();
            }
        }

        let ref mut out = String::new();
        for n in 1 ..= u8::MAX {
            use ::core::fmt::Write;

            #[with]
            let roman = Roman(n).to_str();
            writeln!(out, "{:3} = {}", n, roman).unwrap();
        }
        assert_eq!(out, include_str!("romans.txt"));
    }
}
