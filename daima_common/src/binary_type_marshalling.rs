pub type SerilizedAsBinaries = Result<Vec<u8>, rmp_serde::encode::Error>;
pub type DeserilizeAsBinaries<T> = Result<T, rmp_serde::decode::Error>;

pub fn serilize<T>(value: &T) -> SerilizedAsBinaries
where
    T: serde::ser::Serialize,
{
    rmp_serde::to_vec(&value)
}

pub fn deserilize<'a, T>(value: &'a [u8]) -> DeserilizeAsBinaries<T>
where
    T: serde::de::Deserialize<'a>,
{
    rmp_serde::from_slice(&value)
}
