// ------------------------------------------------------------------------------------------------
// Macros ❱ Tree Wrapper
// ------------------------------------------------------------------------------------------------

macro_rules! tree_wrapper {
    () => {
        tree_wrapper!(ParseTree);
    };
    ($tyname: ident) => {
        #[derive(Clone, Debug)]
        pub struct $tyname<'a> {
            source: Cow<'a, str>,
            tree: Tree,
        }
    };
}

macro_rules! tree_wrapper_impl {
    ($rootname: ident, $roottype: ty) => {
        tree_wrapper_impl!(ParseTree, $rootname, $roottype);
    };
    ($tyname: ident, $rootname: ident, $roottype: ty) => {
        impl<'a> $tyname<'a> {
            pub(crate) fn new(source: Cow<'a, str>, tree: tree_sitter::Tree) -> Self {
                Self { source, tree }
            }

            #[inline(always)]
            pub fn source(&self) -> &Cow<'a, str> {
                &self.source
            }

            #[inline(always)]
            pub(crate) fn node(&self) -> tree_sitter::Node<'_> {
                self.tree.root_node()
            }

            pub fn $rootname(&self) -> $roottype {
                <$roottype>::new(&self.source, self.node())
            }

            #[allow(dead_code)]
            fn save_parse_graph<P>(&self, path: P)
            where
                P: AsRef<Path>,
            {
                let file = File::create(path).unwrap();
                self.tree.print_dot_graph(&file)
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper
// ------------------------------------------------------------------------------------------------

macro_rules! node_wrapper {
    ($tyname: ident) => {
        #[derive(Clone, Debug)]
        pub struct $tyname<'a> {
            source: &'a Cow<'a, str>,
            node: tree_sitter::Node<'a>,
        }
    };
}

macro_rules! node_wrapper_impl {
    ($tyname: ident) => {
        impl<'a> $tyname<'a> {
            fn new(source: &'a Cow<'a, str>, node: tree_sitter::Node<'a>) -> Self {
                Self { source, node }
            }

            #[allow(dead_code)]
            #[inline(always)]
            fn node(&self) -> &tree_sitter::Node<'a> {
                &self.node
            }
        }

        impl<'a> NodeWrapper<'a> for $tyname<'a> {
            #[inline(always)]
            fn text(&self) -> Result<&'a str, $crate::error::Error> {
                Ok(self.node.utf8_text(self.source.as_bytes())?)
            }

            #[inline(always)]
            fn start_byte(&self) -> usize {
                self.node.start_byte()
            }

            #[inline(always)]
            fn end_byte(&self) -> usize {
                self.node.end_byte()
            }

            #[inline(always)]
            fn start_position(&self) -> tree_sitter::Point {
                self.node.start_position()
            }

            #[inline(always)]
            fn end_position(&self) -> tree_sitter::Point {
                self.node.end_position()
            }
        }
    };
}

macro_rules! node_as_str_impl {
    ($tyname: ident) => {
        impl AsRef<str> for $tyname<'_> {
            fn as_ref(&self) -> &str {
                self.text().unwrap()
            }
        }
        impl Display for $tyname<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }
    };
}

macro_rules! node_has_annotations_impl {
    ($tyname: ident) => {
        impl<'a> $tyname<'a> {
            node_wrapper_child_list!(annotations, "annotation", Annotation<'a>);
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper ❱ Children
// ------------------------------------------------------------------------------------------------

macro_rules! node_wrapper_child_list {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> Vec<$rettype> {
            let mut results: Vec<$rettype> = Default::default();
            let parent_node = self.node();
            let mut cursor = parent_node.walk();
            cursor.goto_first_child();
            for _ in 0..parent_node.named_child_count() {
                while !cursor.node().is_named() {
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                if cursor.node().kind() == $namestr {
                    results.push(<$rettype>::new(self.source, cursor.node()));
                }
                cursor.goto_next_sibling();
            }
            results
        }
    };
    ($name: ident, [ $( $namestr: literal ),+ ], $rettype: ty) => {
        pub fn $name(&self) -> Vec<$rettype> {
            let mut results: Vec<$rettype> = Default::default();
            let parent_node = self.node();
            let mut cursor = parent_node.walk();
            cursor.goto_first_child();
            for _ in 0..parent_node.named_child_count() {
                while !cursor.node().is_named() {
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                if [$( $namestr ),+].contains(&cursor.node().kind()) {
                    results.push(<$rettype>::new(self.source, cursor.node()));
                }
                cursor.goto_next_sibling();
            }
            results
        }
    };
    ($name: ident, $rettype: ty) => {
        pub fn $name(&self) -> Vec<$rettype> {
            let mut results: Vec<$rettype> = Default::default();
            let parent_node = self.node();
            let mut cursor = parent_node.walk();
            cursor.goto_first_child();
            for _ in 0..parent_node.named_child_count() {
                while !cursor.node().is_named() {
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                results.push(<$rettype>::new(self.source, cursor.node()));
                cursor.goto_next_sibling();
            }
            results
        }
    };
}

macro_rules! node_wrapper_child_single_opt {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> Option<$rettype> {
            let parent_node = self.node();
            let mut cursor = parent_node.walk();
            cursor.goto_first_child();
            for _ in 0..parent_node.named_child_count() {
                while !cursor.node().is_named() {
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                if cursor.node().kind() == $namestr {
                    return Some(<$rettype>::new(self.source, cursor.node()));
                }
                cursor.goto_next_sibling();
            }
            None
        }
    };
}

macro_rules! node_wrapper_child_single {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> $rettype {
            let parent_node = self.node();
            let mut cursor = parent_node.walk();
            cursor.goto_first_child();
            for _ in 0..parent_node.named_child_count() {
                while !cursor.node().is_named() {
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                if cursor.node().kind() == $namestr {
                    return <$rettype>::new(self.source, cursor.node());
                }
                cursor.goto_next_sibling();
            }
            unreachable!();
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper ❱ Fields
// ------------------------------------------------------------------------------------------------

macro_rules! node_wrapper_field_single {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> $rettype {
            // unwrap is safe here as this assumes a required field
            <$rettype>::new(
                self.source,
                self.node().child_by_field_name($namestr).unwrap(),
            )
        }
    };
}

macro_rules! node_wrapper_field_single_opt {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> Option<$rettype> {
            if let Some(child_node) = self.node().child_by_field_name($namestr) {
                Some(<$rettype>::new(self.source, child_node))
            } else {
                None
            }
        }
    };
}

macro_rules! node_wrapper_field_single_from_str {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> $rettype {
            // unwrap is safe here as this assumes a required field
            let node = self.node().child_by_field_name($namestr).unwrap();
            let text = node.utf8_text(self.source.as_bytes()).unwrap();
            <$rettype>::from_str(text).unwrap()
        }
    };
}

macro_rules! node_wrapper_field_single_from_str_opt {
    ($name: ident, $namestr: literal, $rettype: ty) => {
        pub fn $name(&self) -> Option<$rettype> {
            if let Some(child_node) = self.node().child_by_field_name($namestr) {
                let text = child_node.utf8_text(self.source.as_bytes()).unwrap();
                Some(<$rettype>::from_str(text).unwrap())
            } else {
                None
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! node_wrapper_has_field {
    ($name: ident, $namestr: literal) => {
        pub fn $name(&self) -> bool {
            self.node().child_by_field_name($namestr).is_some()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Choice Wrapper
// ------------------------------------------------------------------------------------------------

macro_rules! choice_wrapper {
    ($tyname: ident, $($varname:ident),+) => {
        #[derive(Clone, Debug)]
        pub enum $tyname<'a> {
            $(
                $varname($varname<'a>)
            ),+
        }
    };
}

macro_rules! choice_wrapper_impl {
    ($ndname: literal => $tyname: ident, $($varname: literal => $vartype: ident),+) => {
        impl<'a> $tyname<'a> {
            fn new(source: &'a Cow<'a, str>, node: tree_sitter::Node<'a>) -> Self {
                assert_eq!(node.kind(), $ndname);
                Self::new_inner(source, node.named_child(0).unwrap())
            }
            _choice_wrapper_new_impl!($tyname, new_inner, $($varname => $vartype),+);
            _choice_wrapper_text_impl!($tyname, $($vartype),+);
        }
    };
    ($tyname: ident, $($varname: literal => $vartype: ident),+) => {
        impl<'a> $tyname<'a> {
            _choice_wrapper_new_impl!($tyname, new, $($varname => $vartype),+);
            _choice_wrapper_text_impl!($tyname, $($vartype),+);
        }
    };
}

macro_rules! _choice_wrapper_new_impl {
    ($tyname: ident, $fnname: ident, $($varname: literal => $vartype: ident),+) => {
        fn $fnname(source: &'a Cow<'a, str>, node: tree_sitter::Node<'a>) -> Self {
            $(
                if node.kind() == $varname {
                    return Self::$vartype(<$vartype<'a>>::new(source, node));
                }
            )+
                unreachable!("unexpected node kind: {:?}", node.kind())
        }
    };
}

macro_rules! _choice_wrapper_text_impl {
    ($tyname: ident, $($vartype: ident),+) => {
        pub fn text(&self) -> Result<&'a str, $crate::error::Error> {
            match self {
                $(
                    Self::$vartype(v) => v.text(),
                )+
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To String
// ------------------------------------------------------------------------------------------------

macro_rules! write_to_string {
    ($outer:ident, $inner:ident) => {
        pub fn $outer(tree: &$crate::api::ParseTree<'_>) -> Result<String, $crate::error::Error> {
            let mut buffer = ::std::io::Cursor::new(Vec::new());
            $inner(tree, &mut buffer)?;
            Ok(String::from_utf8(buffer.into_inner())?)
        }
    };
    ($outer:ident, $inner:ident, $formtype:ty) => {
        pub fn $outer(
            tree: &$crate::api::ParseTree<'_>,
            format: $formtype,
        ) -> Result<String, $crate::error::Error> {
            let mut buffer = ::std::io::Cursor::new(Vec::new());
            $inner(tree, &mut buffer, format)?;
            Ok(String::from_utf8(buffer.into_inner())?)
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To File
// ------------------------------------------------------------------------------------------------

macro_rules! write_to_file {
    ($outer:ident, $inner:ident) => {
        pub fn $outer<P>(
            tree: &$crate::api::ParseTree<'_>,
            path: P,
        ) -> Result<(), $crate::error::Error>
        where
            P: AsRef<::std::path::Path>,
        {
            let mut file = ::std::fs::File::create(path.as_ref())?;
            $inner(tree, &mut file)?;
            Ok(())
        }
    };
    ($outer:ident, $inner:ident, $formtype:ty) => {
        pub fn $outer<P>(
            tree: &$crate::api::ParseTree<'_>,
            path: P,
            format: $formtype,
        ) -> Result<(), $crate::error::Error>
        where
            P: AsRef<::std::path::Path>,
        {
            let mut file = ::std::fs::File::create(path.as_ref())?;
            $inner(tree, &mut file, format)?;
            Ok(())
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To File
// ------------------------------------------------------------------------------------------------

macro_rules! print_to_stdout {
    ($outer:ident, $inner:ident) => {
        pub fn $outer(tree: &$crate::api::ParseTree<'_>) -> Result<(), $crate::error::Error> {
            $inner(tree, &mut ::std::io::stdout())?;
            Ok(())
        }
    };
    ($outer:ident, $inner:ident, $formtype:ty) => {
        pub fn $outer(
            tree: &$crate::api::ParseTree<'_>,
            format: $formtype,
        ) -> Result<(), $crate::error::Error> {
            $inner(tree, &mut ::std::io::stdout(), format)?;
            Ok(())
        }
    };
}
