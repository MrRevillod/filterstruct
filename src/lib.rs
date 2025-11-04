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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct UserFilter {
        pub id: Option<Uuid>,
        pub email: Option<String>,
        pub search: Option<String>,
        pub page: Option<u64>,
    }

    macro_rules! user_filter {
        ($($field:ident $(: $value:expr)?),* $(,)?) => {
            crate::filter!(UserFilter, { $($field $(: $value)?),* })
        };
    }

    #[test]
    fn test_filter_with_id() {
        let id = Uuid::new_v4();
        let filter = user_filter! {
            id: id.clone(),
        };
        assert_eq!(filter.id, Some(id));
        assert_eq!(filter.email, None);
        assert_eq!(filter.search, None);
        assert_eq!(filter.page, None);
    }

    #[test]
    fn test_filter_shorthand_email() {
        let email = String::from("some@domain.com");
        let filter = user_filter! {
            email,
            page: 2,
        };
        assert_eq!(filter.id, None);
        assert_eq!(filter.email, Some(String::from("some@domain.com")));
        assert_eq!(filter.search, None);
        assert_eq!(filter.page, Some(2));
    }

    #[test]
    fn test_filter_multiple_fields() {
        let id = Uuid::new_v4();
        let search = String::from("query");
        let filter = user_filter! {
            id: id.clone(),
            search,
            page: 5,
        };
        assert_eq!(filter.id, Some(id));
        assert_eq!(filter.email, None);
        assert_eq!(filter.search, Some(String::from("query")));
        assert_eq!(filter.page, Some(5));
    }

    #[test]
    fn test_filter_empty() {
        let filter = user_filter! {};
        assert_eq!(filter.id, None);
        assert_eq!(filter.email, None);
        assert_eq!(filter.search, None);
        assert_eq!(filter.page, None);
    }
}
