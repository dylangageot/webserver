pub mod method;
pub mod status;
pub mod version;

pub use method::Method;
pub use status::Status;
pub use version::Version;

use std::collections::HashMap;
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;
pub type Reason = String;

pub mod message;
pub struct Response {}
