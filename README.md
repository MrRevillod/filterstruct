
# FilterStruct

Simple macro for create instances of structs with more ergonomic syntax.

## How to use

Define your struct as usual:

```rust
#[derive(Default, Clone)]
pub struct UserFilter {
    pub id: Option<Uuid>,
    pub email: Option<String>,
    pub search: Option<String>,
    pub page: u64,
}
```

Then, define your filter using the `filter!` macro:

```rust
macro_rules! user_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(UserFilter, { $($field $(: $value)?),* })
    };
}
```

Now you can create instances of `UserFilter` using the `user_filter!` macro:

```rust
let filter = user_filter! {
    id: Uuid::new_v4(),
}
```

This will create a `UserFilter` instance with the `id` field set to a new UUID, and all other fields set to their default values.