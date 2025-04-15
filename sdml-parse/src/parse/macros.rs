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

macro_rules! check_node {
    ($context:expr, $rule_name:expr, $node:expr) => {
        check_node!($context, $rule_name, $node, "")
    };
    ($context:expr, $rule_name:expr, $node:expr, $node_type:expr) => {
        $context.check_if_error(&$node, $rule_name)?;
        if !$node_type.is_empty() && $node.kind() != $node_type {
            unexpected_node!($context, $rule_name, $node, [$node_type,]);
        }
    };
}

macro_rules! node_field_named {
    ($context:expr, $rule_name:expr, $node:expr, $field:expr) => {
        node_field_named!($context, $rule_name, $node, $field, "")
    };
    ($context:expr, $rule_name:expr, $node:expr, $field:expr, $node_type:expr) => {
        match $node.child_by_field_name($field) {
            Some(child) => {
                check_node!($context, $rule_name, child, $node_type);
                child
            }
            None => {
                missing_node!($context, $rule_name, $node, $node_type, $field);
            }
        }
    };
}

macro_rules! optional_node_field_named {
    ($context:expr, $rule_name:expr, $node:expr, $field:expr) => {
        optional_node_field_named!($context, $rule_name, $node, $field, "")
    };
    ($context:expr, $rule_name:expr, $node:expr, $field:expr, $node_type:expr) => {
        if let Some(child) = $node.child_by_field_name($field) {
            check_node!($context, $rule_name, child, $node_type);
            Some(child)
        } else {
            None
        }
    };
}
