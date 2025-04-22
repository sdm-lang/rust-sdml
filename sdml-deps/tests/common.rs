#[macro_export]
macro_rules! generator {
    ($fn_name: ident, $generator: expr, $options: expr, $setup_fn: expr) => {
        fn $fn_name(
            module: &::sdml_core::model::modules::Module,
            cache: &::sdml_core::store::InMemoryModuleCache,
        ) -> String {
            let _ = $setup_fn();
            let mut generator = $generator;
            let options = $options;
            generator
                .generate_to_string(module, cache, options, None)
                .unwrap()
        }
    };
    ($fn_name: ident, $generator: expr, $options: expr) => {
        fn $fn_name(
            module: &::sdml_core::model::modules::Module,
            cache: &::sdml_core::store::InMemoryModuleCache,
        ) -> String {
            let mut generator = $generator;
            let options = $options;
            generator
                .generate_to_string(module, cache, options, None)
                .unwrap()
        }
    };
    ($generator: expr, $options: expr, $setup_fn: expr) => {
        generator! {
            generate_to_string, $generator, $options, $setup_fn
        }
    };
    ($generator: expr, $options: expr) => {
        generator! {
            generate_to_string, $generator, $options
        }
    };
}
