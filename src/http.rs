pub mod method;
pub mod status;
pub mod version;
pub mod message;
pub mod body;
pub mod headers;
pub mod index;

pub use method::Method;
pub use status::Status;
pub use version::Version;
pub use message::Message;
pub use message::Type;
pub use body::Body;
pub use headers::Headers;

pub type Url = String;