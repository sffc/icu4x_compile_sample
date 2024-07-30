extern crate alloc;

pub mod provider {
    pub struct Baked;

    const _: () = {
        use experimental_data::*;
        pub mod icu {
            pub use icu_experimental as experimental;
            pub use experimental_data::icu_locale as locale;
        }
        make_provider!(Baked);

        impl_units_display_name_v1_marker!(Baked);
    };
}
