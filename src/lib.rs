//:T -------- Library Modules Definitions -------- T://
#[path = "lib/colors"]
pub mod colors {
    include!("lib/colors.rs");
    #[path = "ansi"]
    pub mod ansi {
        pub mod ansi16;
        pub mod ansi256;
    }
    pub mod rgb;
}
#[path = "lib/prefixes"]
pub mod prefixes {
    include!("lib/prefixes.rs");
    pub mod ansi16;
    pub mod ansi256;
}
//:T ------ End Library Modules Definitions ------ T://
