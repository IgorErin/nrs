//! Defines NFSv3 [`RmDir`] interface.

use crate::vfs;

/// Success result.
pub struct Success {
    /// Weak cache consistency data for the directory.
    pub wcc_data: vfs::WccData,
}

/// Fail result.
pub struct Fail {
    /// Error on failure.
    pub error: vfs::Error,
    /// Weak cache consistency data for the directory from [`Args::object`].
    pub dir_wcc: vfs::WccData,
}

type Result = std::result::Result<Success, Fail>;

/// Defines callback to pass [`RmDir::rm_dir`] result into.

pub trait Promise {
    fn keep(promise: Result) -> impl std::future::Future<Output = ()> + Send;
}

/// [`RmDir::rm_dir`] arguments.
pub struct Args {
    /// A [`vfs::DirOpArgs`] structure identifying the directory entry
    /// to be removed.
    pub object: vfs::DirOpArgs,
}

pub trait RmDir {
    /// Removes (deletes) a subdirectory from a directory.
    ///
    /// On some servers, the filename, ".", is illegal. These servers will return
    /// the error, [`vfs::Error::InvalidArgument`].
    ///
    /// On some servers, the filename, "..", is illegal. These servers will return
    /// the error, [`vfs::Error::Exist`].
    fn rm_dir(
        &self,
        args: Args,
        promise: impl Promise,
    ) -> impl std::future::Future<Output = ()> + Send;
}
