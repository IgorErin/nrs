//! Defines NFSv3 [`Create`] interface.

use crate::vfs;

use super::file;

pub const VERIFY_LEN: usize = 8;

/// Opaque byte array of `VERIFY_LEN` size used in `How`
pub struct Verifier(pub [u8; VERIFY_LEN]);

/// Describes how the server is to handle the file creation.
pub enum How {
    /// Means that the file should be created without checking
    /// for the existence of a duplicate file in the same
    /// directory with initial attributes for the file.
    Unchecked(super::set_attr::NewAttr),
    /// Specifies that the server should check for the presence
    /// of a duplicate file before performing the create and
    /// should fail the request with [`vfs::Error::Exist`] if a
    /// duplicate file exists.
    ///
    /// If the file does not exist, the request is performed as
    /// described for [`How::Unchecked`].
    Guarded(super::set_attr::NewAttr),
    /// Specifies that the server is to follow exclusive creation
    /// semantics, using the verifier to ensure exclusive creation
    /// of the target. No attributes provided in this case, since the
    /// server may use the target file metadata to store the [`Verifier`].
    Exclusive(Verifier),
}

/// Success result.
pub struct Success {
    /// The file handle of the newly created regular file.
    pub file: Option<file::Handle>,
    /// The attributes of the regular file just created.
    pub attr: Option<file::Attr>,
    /// Weak cache consistency data for the directory of creation.
    pub wcc_data: vfs::WccData,
}

/// Fail result.
pub struct Fail {
    /// Error on failure.
    pub error: vfs::Error,
    /// Weak cache consistency data for the directory.
    pub wcc_data: vfs::WccData,
}

type Result = std::result::Result<Success, Fail>;

/// Defines callback to pass [`Create::create`] result into.

pub trait Promise {
    fn keep(promise: Result) -> impl std::future::Future<Output = ()> + Send;
}

/// [`Create::create`] arguments.
pub struct Args {
    /// The location of the file to be created.
    pub object: vfs::DirOpArgs,
    /// The file creation mode. See [`How`] documentation.
    pub how: How,
}

pub trait Create {
    /// Creates a [`file::Type::Regular`] file.
    fn create(
        &self,
        args: Args,
        promise: impl Promise,
    ) -> impl std::future::Future<Output = ()> + Send;
}
