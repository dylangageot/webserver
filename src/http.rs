pub mod method;
pub mod version;

pub use method::Method;
pub use version::Version;

use std::{collections::HashMap};
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;

pub mod request;
pub struct Response {}
