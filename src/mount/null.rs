//! Defines Mount version 3 [`Null`] interface (Procedure 0).
//!
//! as defined in RFC 1813 section 5.2.0.
//! <https://datatracker.ietf.org/doc/html/rfc1813#section-5.2.0>.

/// Defines callback to pass [`Null::null`] result into.

pub trait Promise {
    fn keep() -> impl std::future::Future<Output = ()> + Send;
}

pub trait Null {
    /// Does not do any work. It is made available to allow server response
    /// testing and timing.
    ///
    /// The procedure takes no MOUNT protocol arguments and returns no MOUNT protocol response.
    /// By convention, the procedure should never require any authentication.
    fn null(&self, promise: impl Promise) -> impl std::future::Future<Output = ()> + Send;
}
