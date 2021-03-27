use arc_rs::{Result, Value};

use arc_rs::utils::parse_toml;
use std::fs::{self, read_to_string};

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let ast = parse_toml(include_str!(concat!($file_name, ".toml"))).unwrap();
        assert_eq!(include_str!(concat!($file_name, ".out.arc")), format!("{:#?}", Value::from(ast)))
    }
    };
}

run_test![example, hard, hard_unicode,];
