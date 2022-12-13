pub mod method;
pub mod status;
pub mod version;
pub mod message;

pub use method::Method;
pub use status::Status;
pub use version::Version;
pub use message::Message;

use std::collections::HashMap;
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;
pub type Reason = String;

pub struct Response {}
