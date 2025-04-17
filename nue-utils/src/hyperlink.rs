pub trait Ext {
    fn hyperlink(&self, url: impl AsRef<str>) -> String;
}

impl<T> Ext for T
where
    T: std::fmt::Display,
{
    fn hyperlink(&self, url: impl AsRef<str>) -> String {
        format!(
            "\u{001B}]8;;{}\u{0007}{self}\u{001B}]8;;\u{0007}",
            url.as_ref()
        )
    }
}
