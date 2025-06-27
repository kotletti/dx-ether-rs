pub trait PersonalSignPort: Send + Sync {
    type Output;
    type Error;

    fn personal_sign(
        &self,
        message: &str,
        address: &str,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
