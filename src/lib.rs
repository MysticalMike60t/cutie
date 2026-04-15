//:T -------- Library Modules Definitions -------- T://
#[path = "lib/colors"]
pub mod colors {
    include!("lib/colors.rs");
    #[path = "ansi"]
    pub mod ansi {
        use cfg_if::cfg_if;
        cfg_if! {
            if #[cfg(feature = "ansi24bit")] {
                pub mod ansi256 {
                    include!("lib/colors/ansi/ansi24bit.rs");
                }
            } else if #[cfg(feature = "ansi256")] {
                pub mod ansi256 {
                    include!("lib/colors/ansi/ansi256.rs");
                }
            } else if #[cfg(feature = "ansi16")] {
                pub mod ansi16 {
                    include!("lib/colors/ansi/ansi16.rs");
                }
            } else {
                pub mod ansi16 {
                    include!("lib/colors/ansi/ansi16.rs");
                }
            }
        }
    }
    pub mod conv;
    pub mod rgb;
}
#[path = "lib/prefixes"]
pub mod prefixes {
    include!("lib/prefixes.rs");

    use cfg_if::cfg_if;
    cfg_if! {
        if #[cfg(feature = "ansi24bit")] {
            pub mod ansi24bit {
                include!("lib/prefixes/ansi24bit.rs");
            }
        } else if #[cfg(feature = "ansi256")] {
            pub mod ansi256 {
                include!("lib/prefixes/ansi256.rs");
            }
        } else if #[cfg(feature = "ansi16")] {
            pub mod ansi16 {
                include!("lib/prefixes/ansi16.rs");
            }
        } else {
            pub mod ansi16 {
                include!("lib/colors/ansi/ansi16.rs");
            }
        }
    }
}
#[path = "lib/tty"]
pub mod tty {
    pub mod tput;
}
//:T ------ End Library Modules Definitions ------ T://
