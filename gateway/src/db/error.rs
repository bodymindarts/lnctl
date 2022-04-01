pub type DbResult<T> = Result<T, DbError>;
pub struct DbError(pub sled::Error);

impl From<sled::Error> for DbError {
    fn from(err: sled::Error) -> Self {
        DbError(err)
    }
}
