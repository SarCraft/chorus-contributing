use std::any::Any;
use std::fmt::Debug;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait BlockComponent: Debug + AsAny + Send + Sync + 'static {}
