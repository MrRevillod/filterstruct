#[macro_export]
macro_rules! filter {
    ($struct_name:ident, { $($field:ident $(: $value:expr)?),* $(,)? }) => {
        {
            #[allow(unused_mut)]
            let mut filter = $struct_name::default();
            $(
                filter.$field = Some($crate::filter!(@assign $field $(: $value)?));
            )*
            filter
        }
    };

    (@assign $field:ident : $value:expr) => { $value };
    (@assign $field:ident) => { $field };

    (@assign_user $filter:ident, page : $value:expr) => {
        $filter.page = $value;
    };
    (@assign_user $filter:ident, page) => {
        $filter.page = page;
    };
    (@assign_user $filter:ident, $field:ident : $value:expr) => {
        $filter.$field = Some($value);
    };
    (@assign_user $filter:ident, $field:ident) => {
        $filter.$field = Some($field);
    };
}
