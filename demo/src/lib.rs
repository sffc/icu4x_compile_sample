extern crate alloc;

mod structs;

pub mod provider {
    pub struct Baked;


    const _: () = {
        use experimental_data::*;
        pub mod icu {
            pub use experimental_data::icu_locale as locale;

            pub mod experimental {
                pub mod dimension {
                    pub mod provider {
                        pub mod units {
                            pub use crate::structs::*;
                        }
                    }
                }
            }
        }
        make_provider!(Baked);

        impl_units_display_name_v1_marker!(Baked);
    };
}
