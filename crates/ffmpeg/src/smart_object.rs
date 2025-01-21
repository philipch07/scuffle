#[derive(Debug)]
pub struct SmartPtr<T>(SmartObject<*mut T>);

#[derive(Debug)]
pub struct SmartObject<T> {
    value: Option<T>,
    destructor: fn(&mut T),
}

impl<T> SmartObject<T> {
    pub(crate) fn new(value: T, destructor: fn(&mut T)) -> Self {
        Self {
            value: Some(value),
            destructor,
        }
    }

    pub(crate) fn set_destructor(&mut self, destructor: fn(&mut T)) {
        self.destructor = destructor;
    }

    pub(crate) fn into_inner(mut self) -> T {
        self.value.take().unwrap()
    }

    fn inner(&self) -> T
    where
        T: Copy,
    {
        self.value.unwrap()
    }

    fn mut_inner(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
    }
}

impl<T> std::ops::Deref for SmartObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<T> std::ops::DerefMut for SmartObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<T> Drop for SmartObject<T> {
    fn drop(&mut self) {
        if let Some(mut value) = self.value.take() {
            (self.destructor)(&mut value);
        }
    }
}

impl<T> AsRef<T> for SmartObject<T> {
    fn as_ref(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}

impl<T> AsMut<T> for SmartObject<T> {
    fn as_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
    }
}

impl<T> SmartPtr<T> {
    /// Safety: The pointer must be valid.
    pub unsafe fn wrap(ptr: *mut T, destructor: fn(&mut *mut T)) -> Self {
        Self(SmartObject::new(ptr, destructor))
    }

    /// Safety: The pointer must be valid.
    pub unsafe fn wrap_non_null(ptr: *mut T, destructor: fn(&mut *mut T)) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self::wrap(ptr, destructor))
        }
    }

    pub(crate) fn set_destructor(&mut self, destructor: fn(&mut *mut T)) {
        self.0.set_destructor(destructor);
    }

    pub(crate) fn into_inner(self) -> *mut T {
        self.0.into_inner()
    }

    pub(crate) fn as_deref(&self) -> Option<&T> {
        // Safety: The pointer is valid.
        unsafe { self.0.inner().as_ref() }
    }

    pub(crate) fn as_deref_mut(&mut self) -> Option<&mut T> {
        // Safety: The pointer is valid.
        unsafe { self.0.inner().as_mut() }
    }

    /// Panics if the pointer is null.
    pub(crate) fn as_deref_except(&self) -> &T {
        self.as_deref().expect("deref is null")
    }

    /// Panics if the pointer is null.
    pub(crate) fn as_deref_mut_except(&mut self) -> &mut T {
        self.as_deref_mut().expect("deref is null")
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.0.inner()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.0.inner()
    }

    pub(crate) fn as_mut(&mut self) -> &mut *mut T {
        self.0.mut_inner()
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use crate::smart_object::{SmartObject, SmartPtr};

    #[test]
    fn test_smart_object_as_ref() {
        let smart_object = SmartObject::new(42, |_value: &mut i32| {});
        let as_ref_value: &i32 = smart_object.as_ref();

        assert_eq!(*as_ref_value, 42, "Expected `as_ref` to return a reference to the value");
    }

    #[test]
    fn test_smart_object_as_mut() {
        let mut smart_object = SmartObject::new(42, |_value: &mut i32| {});
        let as_mut_value: &mut i32 = smart_object.as_mut();
        *as_mut_value = 100;

        assert_eq!(*smart_object, 100, "Expected `as_mut` to allow modifying the value");
    }

    #[test]
    fn test_smart_ptr_wrap_non_null_is_null() {
        // no-op destructor function
        fn noop_destructor<T>(_ptr: &mut *mut T) {}
        let ptr: *mut i32 = std::ptr::null_mut();
        let result = unsafe { SmartPtr::wrap_non_null(ptr, noop_destructor) };

        assert!(result.is_none(), "Expected `wrap_non_null` to return None for a null pointer");
    }
}
