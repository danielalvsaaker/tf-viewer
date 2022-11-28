mod authorizer;
mod issuer;
mod registrar;
pub mod scopes;

use tokio::sync::MutexGuard;

pub struct Guard<'a, T> {
    inner: MutexGuard<'a, T>,
}

impl<'a, T> From<MutexGuard<'a, T>> for Guard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        Self { inner: guard }
    }
}

impl<T> std::ops::Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
