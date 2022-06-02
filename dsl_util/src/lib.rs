pub const NULL_STR: *const i8 = c_str!("");

#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
    ($target: expr, $pat: path, 2) => {{
        if let $pat(a, b) = $target {
            // #1
            (a, b)
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
    ($target: expr, $pat: path, 3) => {{
        if let $pat(a, b, c) = $target {
            // #1
            (a, b, c)
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}
