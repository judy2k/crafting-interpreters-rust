macro_rules! ast_node {
    ( $root:ident, $($name:ident -> $($field:ident : $type:ty), *) ; * $(;)?) => {
        #[derive(Debug)]
        enum $root {
            $($name($name),)*
        }

        paste! {
            pub trait Visitor<R> {
                $(fn [< visit_ $name:lower _ $root:lower >](&self, [<$root:lower>] : &$name) -> R; )*
            }
        }

        $(
        #[derive(Debug)]
        pub struct $name {
            $(
                $field: $type,
            )*
        }

        paste! {
            impl $name {
                pub fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
                    visitor.[< visit_ $name:lower _ $root:lower >](self)
                }
            }
        }
        )*
    }
}

pub(crate) use ast_node;
