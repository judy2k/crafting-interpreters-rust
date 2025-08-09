macro_rules! ast_node {
    ( $root:ident, $($name:ident -> $($field:ident : $type:ty), *) ; * $(;)?) => {
        #[derive(Debug)]
        pub enum $root {
            $($name($name),)*
        }

        paste! {
            pub trait Visitable {
                fn accept<R>(&self, visitor: impl Visitor<R>) -> R;
            }

            pub trait Visitor<R> {
                $(fn [< visit_ $name:lower _ $root:lower >](&self, [<$root:lower>] : &$name) -> R; )*
            }
        }

        $(
        #[derive(Debug)]
        pub struct $name {
            $(
                pub $field: $type,
            )*
        }

        paste! {
            impl $name {
                fn new($($field: $type,)*) -> Self {
                   Self {
                    $($field,)*
                   }
                }
            }

            impl Visitable for $name {
                fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
                    visitor.[< visit_ $name:lower _ $root:lower >](self)
                }
            }
        }
        )*
    }
}

pub(crate) use ast_node;
