// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! emit_diagnostic {
    ($loader: expr, $diagnostic: expr) => {
        $loader.report($diagnostic)?;
    };
}

macro_rules! unexpected_node {
    ($context: expr, $rule_name: expr, $node: expr, [ $($expected: expr, )+ ]) => {
        let expected = [$(
            $expected,
        )+].join(" | ");
        let diagnostic = ::sdml_errors::diagnostics::functions::unexpected_node_kind(
            $context.file_id,
            $node.byte_range(),
            $rule_name,
            expected,
            $node.kind()
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
    ($context: expr, $rule_name: expr, $node: expr, $expected: expr) => {
        let diagnostic = ::sdml_errors::diagnostics::functions::unexpected_node_kind(
            $context.file_id,
            $node.byte_range(),
            $rule_name,
            $expected,
            $node.kind()
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
}

macro_rules! invalid_value_for_node_type {
    ($context: expr, $rule_name: expr, $node: expr, $value: expr, $error: expr) => {
        let diagnostic = ::sdml_errors::diagnostics::functions::invalid_value_for_type_named(
            $context.file_id,
            Some($node.byte_range()),
            $value,
            $node.kind(),
            $error,
        );
        emit_diagnostic!($context.loader, &diagnostic);

        return Err(diagnostic.into())
    };
}

macro_rules! missing_node {
    ($context: expr, $rule_name: expr, $parent_node: expr, $expecting: expr, $field_name: expr) => {
        let diagnostic = ::sdml_errors::diagnostics::functions::missing_node(
            $context.file_id,
            $parent_node.byte_range(),
            $rule_name,
            $expecting,
            Some($field_name),
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

macro_rules! node_field_named {
    ($context:expr, $rule_name:expr, $node:expr, $field:expr, $node_type:expr) => {
        match $node.child_by_field_name($field) {
            Some(child) => {
                $context.check_if_error(&child, $rule_name)?;
                if child.kind() != $node_type {
                    unexpected_node!($context, $rule_name, $node, [$node_type,]);
                }
                child
            }
            None => {
                missing_node!($context, $rule_name, $node, $node_type, $field);
            }
        }
    };
}
