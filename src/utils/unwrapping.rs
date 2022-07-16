#[macro_export]
macro_rules! unwrap_some_else_return {
    ($expression:expr, $return_value:expr) => {
        match $expression {
            Some(e) => e,
            None => return $return_value,
        }
    };
}
