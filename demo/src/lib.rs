extern crate alloc;

pub mod icu {
    pub mod experimental {
        pub mod dimension {
            pub mod provider {
                pub mod units {
                    pub type UnitsDisplayNameV1Marker = UnitsDisplayNameV1<'static>;
                    impl crate::icu_provider::DataMarker for UnitsDisplayNameV1Marker {
                        const INFO: crate::icu_provider::DataMarkerInfo =
                            crate::icu_provider::DataMarkerInfo {
                                fallback_config: (),
                            };
                    }
                    pub struct UnitsDisplayNameV1<'a> {
                        pub patterns:
                            crate::icu::experimental::relativetime::provider::PluralPatterns<'a>,
                    }
                }
            }
        }
        pub mod relativetime {
            pub mod provider {
                pub struct PluralPatterns<'a> {
                    pub strings: crate::icu::plurals::provider::PluralElementsPackedCow<'a>,
                    pub _phantom: core::marker::PhantomData<&'a ()>,
                }
            }
        }
    }
    pub mod locale {
        #[derive(Clone)]
        pub struct DataLocale {}
        impl DataLocale {
            pub fn is_default(&self) -> bool {
                unimplemented!()
            }
        }

        pub mod fallback {
            pub struct LocaleFallbacker;
            impl LocaleFallbacker {
                pub const fn new() -> Self {
                    Self
                }
                pub const fn for_config(&self, _: ()) -> LocaleFallbackerWithConfig {
                    LocaleFallbackerWithConfig {
                        _phantom: core::marker::PhantomData,
                    }
                }
            }
            pub struct LocaleFallbackerWithConfig<'a> {
                _phantom: core::marker::PhantomData<&'a ()>,
            }
            impl LocaleFallbackerWithConfig<'_> {
                pub fn fallback_for(&self, _: crate::icu::locale::DataLocale) -> LocaleFallbackIterator {
                    unimplemented!()
                }
            }
            pub struct LocaleFallbackIterator {}
            impl LocaleFallbackIterator {
                pub fn step(&mut self) {}
                pub fn get(&self) -> crate::icu::locale::DataLocale {
                    unimplemented!()
                }
                pub fn take(self) -> crate::icu::locale::DataLocale {
                    unimplemented!()
                }
            }
        }
    }
    pub mod plurals {
        pub mod provider {
            pub struct PluralElementsPackedCow<'a> {
                pub elements: alloc::borrow::Cow<'a, PluralElementsPackedULE>,
            }
            pub struct PluralElementsPackedULE;
            impl PluralElementsPackedULE {
                pub const fn from_byte_slice_unchecked(_: &[u8]) -> &Self {
                    unimplemented!()
                }
            }
            impl ToOwned for PluralElementsPackedULE {
                type Owned = alloc::boxed::Box<Self>;
                fn to_owned(&self) -> Self::Owned {
                    unimplemented!()
                }
            }
        }
    }
}

pub mod icu_provider_baked {
    pub struct DataStore;
    impl DataStore {
        pub fn get<T>(_: &zerotrie::Data::<T>, _: crate::icu_provider::DataIdentifierBorrowed, _: bool) -> Option<&'static crate::icu_provider::DataPayload<T>> {
            unimplemented!()
        }
    }

    pub mod zerotrie {
        pub struct Data<T: 'static> {
            trie: ZeroTrieSimpleAscii,
            values: &'static [T; 25203],
        }
        pub struct ZeroTrieSimpleAscii {
            store: &'static [u8],
        }
    }
}

pub mod icu_provider {
    pub struct DataError;
    pub enum DataErrorKind {
        IdentifierNotFound,
    }
    impl DataErrorKind {
        pub fn with_req(self, _: DataMarkerInfo, _: DataRequest) -> DataError {
            unimplemented!()
        }
    }
    pub struct DataIdentifierBorrowed {
        pub marker_attributes: (),
        pub locale: crate::icu::locale::DataLocale,
    }
    impl DataIdentifierBorrowed {
        pub fn for_marker_attributes_and_locale(_: (), _: crate::icu::locale::DataLocale) -> Self {
            unimplemented!()
        }
    }
    pub trait DataMarker {
        const INFO: DataMarkerInfo;
    }
    pub struct DataMarkerInfo {
        pub fallback_config: (),
    }
    pub struct DataPayload<M> {
        pub _phantom: core::marker::PhantomData<M>,
    }
    impl<M> DataPayload<M> {
        pub const fn from_static_ref(_: &'static Self) -> Self {
            unimplemented!()
        }
    }
    pub trait DataProvider<M> {
        fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
    }
    pub struct DataRequest {
        pub metadata: DataRequestMetadata,
        pub id: DataIdentifierBorrowed,
    }
    pub struct DataRequestMetadata {
        pub id: DataIdentifierBorrowed,
        pub attributes_prefix_match: bool,
    }
    pub struct DataResponse<M> {
        pub metadata: DataResponseMetadata,
        pub payload: DataPayload<M>,
    }
    pub struct DataResponseMetadata {
        pub locale: Option<crate::icu::locale::DataLocale>,
    }
    impl DataResponseMetadata {
        pub fn default() -> Self {
            unimplemented!()
        }
    }

    pub mod marker {
        #[macro_export]
        macro_rules! impl_data_provider_never_marker {
            ($ty:path) => {};
        }
        pub use impl_data_provider_never_marker;
    }
}

pub struct Baked;

icu_experimental_data::make_provider!(Baked);
icu_experimental_data::impl_units_display_name_v1_marker!(Baked);
