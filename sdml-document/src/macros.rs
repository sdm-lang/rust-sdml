// ------------------------------------------------------------------------------------------------
// Macros ❱ trace entry
// ------------------------------------------------------------------------------------------------

macro_rules! trace_entry {
    ($fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        let tracing_span = ::tracing::trace_span!($fn_name);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{}({arguments})", $fn_name);
    };
    ($type_name: literal, $fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{FULL_NAME}({arguments})");
    };
}
