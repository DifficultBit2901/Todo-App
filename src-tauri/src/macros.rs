macro_rules! string_error {
    ($fun:expr) => {
        $fun.map_err(|err| err.to_string())?
    };
}
