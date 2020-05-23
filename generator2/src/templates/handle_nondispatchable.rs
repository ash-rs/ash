#[macro_export]
macro_rules! handle_nondispatchable {
    ($name: ident, $ty: ident) => {
        handle_nondispatchable!($name, $ty, doc = "");
    };
    ($name: ident, $ty: ident, $doc_link: meta) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        #[$doc_link]
        pub struct $name(u64);

        impl Handle for $name {
            const TYPE: ObjectType = ObjectType::$ty;
            fn as_raw(self) -> u64 { self.0 as u64 }
            fn from_raw(x: u64) -> Self { $name(x as _) }
        }

        impl $name{
            pub const fn null() -> $name{
                $name(0)
            }
        }

        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    }
}
