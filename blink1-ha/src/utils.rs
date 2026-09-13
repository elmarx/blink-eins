use serde::{Deserialize, Serialize};

/// A wrapper type for sensitive values that hides the content when formatted with Debug
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Secret<T>(pub T);

impl<T> std::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("***")
    }
}

impl<T> std::ops::Deref for Secret<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
