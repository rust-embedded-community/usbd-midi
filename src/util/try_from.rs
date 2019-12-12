
/// A version of try from that works on options
/// useful for the bounded ints
pub trait TryFrom<T> : Sized{
    fn try_from(value:T) -> Option<Self>;
}
