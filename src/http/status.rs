
#[derive(EnumString, Display, Debug, PartialEq)]
pub enum Status {
    #[strum(serialize = "200 OK")]
    Ok = 200,
    #[strum(serialize = "202 Accepted")]
    Accepted = 202,
    #[strum(serialize = "404 Not Found")]
    NotFound = 404,
}
