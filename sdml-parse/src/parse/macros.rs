// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! emit_diagnostic {
    ($loader: expr, $diagnostic: expr) => {
        $loader.report($diagnostic)?;
    };
}

macro_rules! unexpected_node {
    ($context: expr, $parse_fn: expr, $node: expr, [ $($expected: expr, )+ ]) => {
        let expected = [$(
            $expected,
        )+].join(" | ");
        let diagnostic = ::sdml_error::diagnostics::unexpected_node_kind(
            $context.file_id,
            $node.start_byte()..$node.end_byte(),
            expected,
            $node.kind()
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
    ($context: expr, $parse_fn: expr, $node: expr, $expected: expr) => {
        let diagnostic = ::sdml_error::diagnostics::unexpected_node_kind(
            $context.file_id,
            $node.start_byte()..$node.end_byte(),
            $expected,
            $node.kind()
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
}

macro_rules! missing_node {
    ($context: expr, $parse_fn: expr, $parent_node: expr, $variable_name: expr, $node_kind: expr) => {
        let diagnostic = ::sdml_error::diagnostics::missing_node(
            $context.file_id,
            $parent_node.start_byte()..$parent_node.end_byte(),
            $node_kind,
            Some($variable_name),
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
}

macro_rules! rule_fn {
    ($name: expr, $node: expr) => {
        const RULE_NAME: &str = $name;
        let tracing_span = ::tracing::trace_span!($name);
        let _enter_span = tracing_span.enter();
        ::tracing::trace!("{}: {:?}", RULE_NAME, $node);
    };
}

#[allow(unused_macros)]
macro_rules! rule_todo {
    ($name: expr) => {
        let msg = format!("Incomplete parse function for rule `{}`", $name);
        ::tracing::error!("{}", msg);
        todo!("{}", msg);
    };
}

macro_rules! rule_unreachable {
    ($name: expr, $cursor: expr) => {
        let msg = format!(
            "Rule `{}` should not have gotten here with node {:?} => {}",
            $name,
            $cursor.node(),
            $cursor.node().to_sexp()
        );
        ::tracing::error!("{}", msg);
        unreachable!("{}", msg);
    };
}

macro_rules! node_child_named {
    ($node: expr, $name: expr, $context: expr, $rule_name: expr) => {
        node_child_named!($node, $name, "dunno", $context, $rule_name)
    };
    ($node: expr, $name: expr, $kind: expr, $context: expr, $rule_name: expr) => {
        match $node.child_by_field_name($name) {
            Some(child) => {
                $context.check_if_error(&child, $rule_name)?;
                child
            }
            None => {
                missing_node!($context, $rule_name, $node, $name, $kind);
            }
        }
    };
}
