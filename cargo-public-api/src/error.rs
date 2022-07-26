#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The API diff is not allowed as per --deny")]
    DiffDenied,

    #[error("The API diff is not allowed as per --deny=added")]
    DiffAddedDenied,

    #[error("The API diff is not allowed as per --deny=changed")]
    DiffChangedDenied,

    #[error("The API diff is not allowed as per --deny=removed")]
    DiffRemoveDenied,
}
