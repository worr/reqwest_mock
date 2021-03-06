// TODO: Implement more conversions.
pub trait IntoBody {
    fn into_body(self) -> Vec<u8>;
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> Vec<u8> {
        self
    }
}

impl<'a> IntoBody for &'a str {
    fn into_body(self) -> Vec<u8> {
        self.bytes().collect()
    }
}
