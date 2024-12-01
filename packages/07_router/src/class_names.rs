#[macro_export]
macro_rules! class_names {
    ($($x:expr),+ $(,)?) => {
        vec![$($x.to_string()),+].join(" ")
    };
}
