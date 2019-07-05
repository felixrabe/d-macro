use std::time::Instant;

/// Only public for use in `d!()`.
pub const RESET: &str = "\x1b[0m";
/// Only public for use in `d!()`.
pub const RED: &str   = "\x1b[31m";
/// Only public for use in `d!()`.
pub const GREY: &str  = "\x1b[90m";

static mut START: Option<Instant> = None;

pub fn d_prn<S: ToString>(s: S) {
    let start = d_start();
    let t = Instant::now() - start;

    eprintln!("{} {}", internal::disp_time(&t), s.to_string());
}

pub fn d_start() -> Instant {
    match unsafe { START } {
        Some(t) => t,
        None => {
            eprintln!();
            eprintln!("{} START", internal::fo(" s", " ms", " µs"));
            let now = Instant::now();
            unsafe { START = Some(now) };
            now
        }
    }
}

pub fn d_end() {
    d_prn("END");
    eprintln!();
}

#[macro_export]
macro_rules! d {
    // Rules beginning with `@` are meant for internal use only.
    // See https://danielkeep.github.io/tlborm/book/pat-internal-rules.html.

    { @ $expr:expr => $val:ident ( $($format_args:tt)+ ) } => {
        // See the source of `dbg!` for why `match` is needed.
        match $expr {
            $val => {
                let args = format!($($format_args)+);
                d!(@raw args);
                $val
            }
        }
    };
    { @raw $expr:expr } => { {
        let thread = format!("[{}]", std::thread::current().name().unwrap_or("???"));
        let pos = format!("{}:{}", file!(), line!());
        let val = $expr;
        $crate::d_prn(format!("{:6} {}  {}{}{}", thread, &val, $crate::GREY, pos, $crate::RESET));
        val
    } };
    { @ $($tt:tt)* } => {
        d!(@raw format!($($tt)*))
    };

    {} => { d! { @() => val ("") } };

    // I wanted to order these variants in the opposite order, but for some
    // unknown-to-me reason doing so would cause the variant `{ #? $val:expr }`
    // to no longer work, as it expects `[` after `#`.
    { #? $val:expr } => { d! { @ $val => val ("{} = {:#?}", stringify!($val), val) } };
    { ? $val:expr }  => { d! { @ $val => val ("{} = {:?}", stringify!($val), val) } };
    { $val:expr }    => { d! { @ $val => val ("{}", val) } };
}

mod internal {
    use std::time::Duration;

    #[rustfmt::skip]
    pub fn disp_time(t: &Duration) -> String {
        let n = t.as_micros();

        let s2 = String::from("  ");
        let s3 = String::from("   ");

        match                (n / 1000_000,       n / 1000 % 1000,      n % 1000) {
            (0,  0,  0) => fo(s2,                 s3,                   "  0"),
            (0,  0, us) => fo(s2,                 s3,                   format!( "{:3}", us)),
            (0, ms, us) => fo(s2,                 format!( "{:3}", ms), format!("{:03}", us)),
            (s, ms, us) => fo(format!("{:2}", s), format!("{:03}", ms), format!("{:03}", us)),
        }
    }

    pub fn fo(s: impl Into<String>, ms: impl Into<String>, us: impl Into<String>) -> String {
        const S: &str = super::RED;
        const MS: &str = super::RESET;
        const US: &str = super::GREY;
        const RST: &str = super::RESET;

        let (s, ms, us) = (s.into(), ms.into(), us.into());

        format!("{s}{}{rst}{ms}{}{rst} {us}{}{rst}", s, ms, us, s = S, ms = MS, us = US, rst = RST)
    }
}
