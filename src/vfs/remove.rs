//! Defines NFSv3 [`Remove`] interface.

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
    /// Weak cache consistency data for the directory, [`Args::object`] dir.
    pub dir_wcc: vfs::WccData,
}

type Result = std::result::Result<Success, Fail>;

/// Defines callback to pass [`Remove::remove`] result into.

pub trait Promise {
    fn keep(promise: Result) -> impl std::future::Future<Output = ()> + Send;
}

/// [`Remove::remove`] arguments.
pub struct Args {
    /// A [`vfs::DirOpArgs`] structure identifying the entry to be removed.
    pub object: vfs::DirOpArgs,
}

pub trait Remove {
    /// Removes (deletes) an entry from a directory.
    fn remove(
        &self,
        args: Args,
        promise: impl Promise,
    ) -> impl std::future::Future<Output = ()> + Send;
}
