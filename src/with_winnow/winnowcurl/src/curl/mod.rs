pub mod parser;
// Winnow-based implementation lives in `parser.rs` and URL types in `crate::url`.

#[macro_export]
macro_rules! new_curl {
    ($identifier:expr) => {
        Curl::new_as_flag($identifier).unwrap()
    };
    ($identifier:expr,$data:expr) => {
        Curl::new(stringify!($identifier), $data).unwrap()
    };
}

// Re-export core types from winnow-based parser for external use.
pub use parser::{Curl, CurlStru};
