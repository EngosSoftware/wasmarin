//! # Common error definition

/// Result type definition.
pub type WasmarinResult<T, E = WasmarinError> = Result<T, E>;

/// Error definition.
#[derive(PartialEq, Eq)]
pub struct WasmarinError(String);

impl std::fmt::Debug for WasmarinError {
  /// Implementation of [Debug] trait for [WasmarinError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl WasmarinError {
  /// Creates a new [WasmarinError].
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().into())
  }
}
