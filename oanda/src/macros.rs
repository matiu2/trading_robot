#[macro_export]
macro_rules! builder_methods {
    ([$($field:ident : $field_type:ty),* $(,)?]) => {
        $(
            pub fn $field(mut self, $field: $field_type) -> Self {
                self.$field = Some($field);
                self
            }
        )*
    };
}
