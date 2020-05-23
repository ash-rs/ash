#[macro_export]
macro_rules! define_handle{
    ($name: ident, $ty: ident) => {
        define_handle!($name, $ty, doc = "");
    };
    ($name: ident, $ty: ident, $doc_link: meta) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        #[$doc_link]
        pub struct $name(*mut u8);
        impl Default for $name {
            fn default() -> $name {
                $name::null()
            }
        }

        impl Handle for $name {
            const TYPE: ObjectType = ObjectType::$ty;
            fn as_raw(self) -> u64 { self.0 as u64 }
            fn from_raw(x: u64) -> Self { $name(x as _) }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name{
            pub const fn null() -> Self{
                $name(::std::ptr::null_mut())
            }
        }

        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Pointer::fmt(&self.0, f)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&self.0, f)
            }
        }
    }
}

