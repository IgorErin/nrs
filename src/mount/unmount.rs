//! Defines Mount version 3 [`Umnt`] interface (Procedure 3).
//!
//! as defined in RFC 1813 section 5.2.3.
//! <https://datatracker.ietf.org/doc/html/rfc1813#section-5.2.3>.

use std::path::PathBuf;

/// Defines callback to pass [`Umnt::umnt`] result into.

pub trait Promise {
    fn keep() -> impl std::future::Future<Output = ()> + Send;
}

/// Arguments for the Unmount operation, containing the path to be unmounted.
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct UnmountArgs(pub PathBuf);

pub trait Umnt {
    /// Removes the mount list entry for the directory that was
    /// previously the subject of a MNT call from this client.
    ///
    /// # Parameters:
    /// * `dirpath` --- a server pathname of a directory.
    ///
    /// AUTH_UNIX authentication or better is required.
    /// There are no MOUNT protocol errors which can be returned from this procedure.
    fn umnt(&self, dirpath: PathBuf) -> impl std::future::Future<Output = ()> + Send;
}
