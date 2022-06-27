// FIXME (#20041): Replace this workaround with real type equality constraints
pub trait TypeEquals<T>: From<T> { }
impl<T> TypeEquals<T> for T { }
