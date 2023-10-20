// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! emit_diagnostic {
    ($files: expr, $diagnostic: expr) => {
        // TODO: parameterize this ---------vvvvvvvvvvvvvvvvvvv
        let writer = ::codespan_reporting::term::termcolor::StandardStream::stderr(
            ::codespan_reporting::term::termcolor::ColorChoice::Always
        );
        let mut config = codespan_reporting::term::Config::default();
        config.chars = ::codespan_reporting::term::Chars::box_drawing();
        emit_diagnostic!($files, $diagnostic, &config => writer);
    };
    // ($files: expr, $diagnostic: expr => $writer: expr) => {
    //     let config = codespan_reporting::term::Config::default();
    //     emit_diagnostic!($files, $diagnostic, &config => $writer);
    // };
    // ($files: expr, $diagnostic: expr, $config: expr) => {
    //     let writer = StandardStream::stderr(ColorChoice::Always);
    //     emit_diagnostic!($files, $diagnostic, $config => writer);
    // };
    ($files: expr, $diagnostic: expr, $config: expr => $writer: expr) => {
        ::codespan_reporting::term::emit(&mut $writer.lock(), &$config, &*$files, &$diagnostic)?
    };
}

macro_rules! unexpected_node {
    ($context: expr, $parse_fn: expr, $node: expr, [ $($expected: expr, )+ ]) => {
        let diagnostic = $crate::error::UNEXPECTED_NODE_KIND.into_diagnostic()
            .with_labels(vec![
                ::codespan_reporting::diagnostic::Label::primary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message($crate::parse::message_expecting_one_of_node(&[
                        $($expected, )+
                    ])),
                ::codespan_reporting::diagnostic::Label::secondary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message($crate::parse::message_found_node($node.kind())),
                ]);

        $context.counts.report(diagnostic.severity);
        emit_diagnostic!($context.loader.files(), diagnostic);

        return Err(::sdml_core::error::unexpected_node_kind(
            $parse_fn,
            [
                $($expected, )+
            ].join(" | "),
            $node.kind(),
            $node.into(),
        ))
    };
    ($context: expr, $parse_fn: expr, $node: expr, $expected: expr) => {
        let diagnostic = $crate::error::UNEXPECTED_NODE_KIND.into_diagnostic()
            .with_labels(vec![
                ::codespan_reporting::diagnostic::Label::primary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message($crate::parse::message_expecting_node($expected)),
                ::codespan_reporting::diagnostic::Label::secondary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message($crate::parse::message_found_node($node.kind())),
                ]);

        $context.counts.report(diagnostic.severity);
        emit_diagnostic!($context.loader.files(), diagnostic);

        return Err(::sdml_core::error::unexpected_node_kind(
            $parse_fn,
            $expected,
            $node.kind(),
            $node.into(),
        ))
    };
}

macro_rules! missing_node {
    ($context: expr, $parse_fn: expr, $parent_node: expr, $variable_name: expr, $node_kind: expr) => {
        let message = format!("Missing a `{}` in grammar variable named `{}`", $node_kind, $variable_name);
        let diagnostic = $crate::error::MISSING_NODE_VARIABLE.into_diagnostic()
            .with_labels(vec![
                ::codespan_reporting::diagnostic::Label::primary($context.file_id, $parent_node.start_byte()..$parent_node.end_byte())
                    .with_message(message),
                ]);

        $context.counts.report(diagnostic.severity);
        emit_diagnostic!($context.loader.files(), diagnostic);

        return Err(::sdml_core::error::missing_node_variable(
            $parse_fn,
            $variable_name,
            $node_kind
        ))
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
