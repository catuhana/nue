use std::fmt;

pub trait Ext {
    fn hyperlink(&self, url: impl ToString) -> String;
}

impl<T> Ext for T
where
    T: fmt::Display,
{
    fn hyperlink(&self, url: impl ToString) -> String {
        format!(
            "\u{001B}]8;;{}\u{0007}{}\u{001B}]8;;\u{0007}",
            url.to_string(),
            self
        )
    }
}
