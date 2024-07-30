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

        impl_long_compact_decimal_format_data_v1_marker!(Baked);
        impl_short_compact_decimal_format_data_v1_marker!(Baked);
        impl_currency_essentials_v1_marker!(Baked);
        impl_units_display_name_v1_marker!(Baked);
        impl_units_essentials_v1_marker!(Baked);
        impl_language_display_names_v1_marker!(Baked);
        impl_digital_duration_data_v1_marker!(Baked);
        impl_locale_display_names_v1_marker!(Baked);
        impl_region_display_names_v1_marker!(Baked);
        impl_script_display_names_v1_marker!(Baked);
        impl_variant_display_names_v1_marker!(Baked);
        impl_percent_essentials_v1_marker!(Baked);
        impl_person_names_format_v1_marker!(Baked);
        impl_long_day_relative_time_format_data_v1_marker!(Baked);
        impl_long_hour_relative_time_format_data_v1_marker!(Baked);
        impl_long_minute_relative_time_format_data_v1_marker!(Baked);
        impl_long_month_relative_time_format_data_v1_marker!(Baked);
        impl_long_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_long_second_relative_time_format_data_v1_marker!(Baked);
        impl_long_week_relative_time_format_data_v1_marker!(Baked);
        impl_long_year_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_day_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_hour_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_minute_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_month_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_second_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_week_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_year_relative_time_format_data_v1_marker!(Baked);
        impl_short_day_relative_time_format_data_v1_marker!(Baked);
        impl_short_hour_relative_time_format_data_v1_marker!(Baked);
        impl_short_minute_relative_time_format_data_v1_marker!(Baked);
        impl_short_month_relative_time_format_data_v1_marker!(Baked);
        impl_short_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_short_second_relative_time_format_data_v1_marker!(Baked);
        impl_short_week_relative_time_format_data_v1_marker!(Baked);
        impl_short_year_relative_time_format_data_v1_marker!(Baked);
        impl_units_info_v1_marker!(Baked);
    };
}
