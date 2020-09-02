pub trait TypeEquals<T>: From<T> { }

impl<T> TypeEquals<T> for T { }
