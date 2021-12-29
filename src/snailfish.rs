use lalrpop_util::lalrpop_mod;

pub use snailfish_syntax::ExprParser;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    snailfish_syntax
);
