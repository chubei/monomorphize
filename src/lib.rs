pub fn generic_fn<T>(_t: T) {}

pub const _: *const () = generic_fn::<i32> as _;

pub struct MonoStruct;

impl MonoStruct {
    pub fn generic_method<T>(&self, _t: T) {}
}

pub const _: *const () = MonoStruct::generic_method::<i32> as _;

pub struct GenericStruct<T>(std::marker::PhantomData<T>);

impl<T> GenericStruct<T> {
    pub fn mono_method(&self) {}

    pub fn generic_method<U>(&self, _u: U) {}
}

pub const _: *const () = GenericStruct::<i32>::mono_method as _;

pub const _: *const () = GenericStruct::<i32>::generic_method::<i32> as _;
