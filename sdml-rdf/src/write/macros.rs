// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! mkiri {
    ($module:ident : $name:ident) => {
        mkiri!($module::MODULE_URL, $module::$name)
    };
    ($base:expr, $name:expr) => {
        Iri::from_str(&format!("{}{}", $base, $name)).unwrap()
    };
}

macro_rules! g_insert {
    ($graph:expr ; $subject:expr, rdf:type, $object:expr) => {
        g_insert!($graph ; $subject, mkiri!(rdf:TYPE), $object)
    };
    ($graph:expr ; $subject:expr, sdml:name, $object:expr) => {
        g_insert!($graph ; $subject, mkiri!(sdml:NAME), $object)
    };
    ($graph:expr ; $subject:expr, sdml:srcLabel => $ctx:expr) => {
        g_insert!(
            $graph ;
            $subject,
            mkiri!(sdml:SRC_LABEL),
            Literal::plain($name.to_string())
        )
    };
    ($graph:expr ; $subject:expr, $predicate:expr, $object:expr) => {
        $graph.insert(Statement::new($subject, $predicate, $object))
    };
    ($graph:expr ; $ctx:expr => rdf:type, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(rdf:TYPE),
            $object
        )
    };
    ($graph:expr ; $ctx:expr => rdf:value, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(rdf:VALUE),
            $object
        )
    };
    ($graph:expr ; $ctx:expr => sdml:srcLabel, $name:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(sdml:SRC_LABEL),
            Literal::plain($name.to_string())
        )
    };
    ($graph:expr ; $ctx:expr => $predicate:expr, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            $predicate,
            $object
        )
    };
    ($graph:expr ; $subject:expr, $predicate:expr => $ctx:expr) => {
        g_insert!(
            $graph ;
            $subject,
            $predicate,
            $ctx.current_subject().to_object()
        )
    };
}

macro_rules! set_current_context {
    ($named:expr, $ctx:expr) => {{
        let new_name = rdftk_iri::Name::from_str($named.name().as_ref()).unwrap();
        let new_subject = $ctx.base_uri.make_name(new_name).unwrap();
        $ctx.push_subject(new_subject);
    }};
}

macro_rules! add_source_span {
    ($owner:expr, $graph:expr, $ctx:expr, $cache:expr) => {
        if let Some(span) = $owner.source_span() {
            span.add_to_graph($graph, $ctx, $cache)?;
        }
    };
}

macro_rules! add_annotations {
    ($owner:expr, $graph:expr, $ctx:expr, $cache:expr) => {
        for annotation in $owner.annotations() {
            annotation.add_to_graph($graph, $ctx, $cache)?;
        }
    };
}

macro_rules! defn_common {
    ($defn:expr, $ctx:expr, $graph:expr, $type_name:expr) => {
        defn_common!($defn, $ctx, $graph, $type_name, sdml::HAS_DEFINITION);
    };
    ($defn:expr, $ctx:expr, $graph:expr, $type_name:expr, $member_property:expr) => {
        if $ctx.has_current_subject() {
            let outer_subject = $ctx.current_subject().clone();
            g_insert!($graph ; outer_subject, mkiri!(sdml::MODULE_URL, $member_property) => $ctx);
        }

        set_current_context!($defn, $ctx);

        g_insert!($graph ; $ctx => rdf:type, mkiri!(sdml::MODULE_URL, $type_name));
        g_insert!($graph ; $ctx => sdml:srcLabel, $defn.name());
    };
}
