pub use binding_powers_impl::__deduplicate_enum;

pub trait Operator {
    fn to_id(&self) -> usize;

    fn prefix_power(&self) -> Option<((), u8)>;

    fn infix_power(&self) -> Option<(u8, u8)>;

    fn postfix_power(&self) -> Option<(u8, ())>;
}

#[derive(Debug, Clone, Copy)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Precedences {
    prefix: Option<((), u8)>,
    infix: Option<(u8, u8)>,
    postfix: Option<(u8, ())>,
}

#[derive(Debug, Clone, Copy)]
pub enum PrecType {
    Infix(Assoc),
    Prefix,
    Postfix,
}

#[derive(Debug, Clone, Copy)]
pub struct BindingPowers<const N: usize>([Precedences; N]);

impl<const N: usize> BindingPowers<N> {
    pub const fn new(kinds: &[(usize, PrecType)]) -> BindingPowers<N> {
        let mut res = [Precedences {
            prefix: None,
            infix: None,
            postfix: None,
        }; N];

        let mut counter = 0;
        let mut idx: usize = 0;

        loop {
            if idx == kinds.len() {
                break;
            }

            let kind_assoc = kinds[idx];

            match kind_assoc.1 {
                PrecType::Infix(assoc) => {
                    counter += 1;
                    let first = counter;
                    counter += 1;
                    let second = counter;

                    let infix = match assoc {
                        Assoc::Left => (first, second),
                        Assoc::Right => (second, first),
                    };
                    res[kind_assoc.0].infix = Some(infix);
                }
                PrecType::Prefix => {
                    let second = {
                        counter += 1;
                        counter
                    };
                    res[kind_assoc.0].prefix = Some(((), second));
                }
                PrecType::Postfix => {
                    let first = {
                        counter += 1;
                        counter
                    };
                    res[kind_assoc.0].postfix = Some((first, ()));
                }
            }

            idx += 1;
        }

        BindingPowers(res)
    }

    pub const fn get_infix(&self, kind: usize) -> Option<(u8, u8)> {
        self.0[kind].infix
    }

    pub const fn get_postfix(&self, kind: usize) -> Option<(u8, ())> {
        self.0[kind].postfix
    }

    pub const fn get_prefix(&self, kind: usize) -> Option<((), u8)> {
        self.0[kind].prefix
    }
}

#[macro_export]
macro_rules! precedences {
    {
        $vis:vis enum $name:ident {
            $(
                #[$($stuff:tt)+]
                $variant:ident
             ),+ $(,)?
        }
    } => {
        mod __precedences_mod {
            use $crate::Assoc::*;
            use $crate::Operator;
            use $crate::BindingPowers;

            $crate::__deduplicate_enum!{ $name $($variant)+ }

            const __BINDING_POWERS: BindingPowers<{$name::__LAST as usize}>= BindingPowers::new(&[
                $(($name::$variant as usize, $crate::precedences!($($stuff)+))),+
            ]);

            impl Operator for $name {
                fn to_id(&self) -> usize {
                    *self as usize
                }

                fn infix_power(&self) -> Option<(u8, u8)> {
                    __BINDING_POWERS.get_infix(*self as usize)
                }

                fn prefix_power(&self) -> Option<((), u8)> {
                    __BINDING_POWERS.get_prefix(*self as usize)
                }

                fn postfix_power(&self) -> Option<(u8, ())> {
                    __BINDING_POWERS.get_postfix(*self as usize)
                }
            }
        }

        #[allow(unused_imports)]
        $vis use __precedences_mod::$name;
    };

    ($prec_type:ident) => {
        $crate::PrecType::$prec_type
    };
    ($prec_type:ident, $assoc:ident) => {
        ($crate::PrecType::$prec_type($assoc))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        precedences! {
            enum Op {
                #[Infix, Left]
                Another,

                #[Prefix]
                Another,

                #[Infix, Right]
                Power,
            }
        }

        assert!(Op::Another.postfix_power().is_none());
        assert_eq!(Op::Another.infix_power(), Some((1, 2)));
        assert_eq!(Op::Another.prefix_power(), Some(((), 3)));

        assert!(Op::Power.prefix_power().is_none());
        assert_eq!(Op::Power.infix_power(), Some((5, 4)));
    }

    #[test]
    fn lua() {
        precedences! {
            enum LuaOp {
                #[Infix, Left]
                Or,

                #[Infix, Left]
                And,

                #[Infix, Left]
                Lt,

                #[Infix, Left]
                Gt,

                #[Infix, Left]
                LtEq,

                #[Infix, Left]
                GtEq,

                #[Infix, Left]
                NotEq,

                #[Infix, Left]
                Eq,

                #[Infix, Left]
                Concat,

                #[Infix, Left]
                Plus,

                #[Infix, Left]
                Minus,

                #[Infix, Left]
                Mul,

                #[Infix, Left]
                Div,

                #[Prefix]
                Not,

                #[Prefix]
                Minus,

                #[Infix, Right]
                Power,
            }
        }
    }
}
