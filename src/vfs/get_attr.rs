//! Defines NFSv3 [`GetAttr`] interface.

use crate::vfs;

use super::file;

/// Defines callback to pass [`GetAttr::get_attr`] result into.

pub trait Promise {
    fn keep(attr: vfs::Result<file::Attr>) -> impl std::future::Future<Output = ()> + Send;
}

/// [`GetAttr::get_attr`] aguments.
pub struct Args {
    /// File handle of an object whose attributes are to be retrieved.
    pub file: file::Handle,
}

pub trait GetAttr {
    /// Retrieves the attributes for a specified file system object.
    fn get_attr(
        &self,
        args: Args,
        promise: impl Promise,
    ) -> impl std::future::Future<Output = ()> + Send;
}
