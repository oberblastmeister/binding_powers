use proc_macro::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{cmp::Ordering, collections::BTreeSet};
use std::hash::{Hash, Hasher};

type Result<T> = std::result::Result<T, String>;

#[doc(hidden)]
#[proc_macro]
pub fn __deduplicate(token_stream: TokenStream) -> TokenStream {
    try_deduplicate(token_stream)
        .unwrap_or_else(|msg| parse_ts(&format!("compile_error!({:?})", msg)))
}

#[derive(Debug)]
struct Ident {
    ident: String,
    span: Span,
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for Ident {}

impl Into<proc_macro::Ident> for Ident {
    fn into(self) -> proc_macro::Ident {
        proc_macro::Ident::new(&self.ident, self.span)
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn try_deduplicate(token_stream: TokenStream) -> Result<TokenStream> {
    // let (cmd, literal) = {
    //     let mut iter = token_stream.into_iter();
    //     let cmd = iter.next().unwrap();
    //     let literal = iter.next().unwrap();
    //     assert!(iter.next().is_none());
    //     (cmd, literal)
    // };

    // return Err(format!("res = {:#?}", token_stream.to_string()));
    let name = token_stream.clone().into_iter().next().unwrap();

    let deduplicated = token_stream
        .into_iter()
        .skip(1)
        .map(|token| {
            Ident {
                ident: token.to_string(),
                span: token.span(),
            }
            // TokenTree::Ident(it) => Some(Ident {
            //     ident: it.to_string(),
            //     span: it.span(),
            // }),
            // _ => None,
        })
        .collect::<BTreeSet<_>>();

    // return Err(format!("res = {:#?}", deduplicated));

    let mut res = TokenStream::new();

    res.extend(parse_ts("#[derive(Debug, Copy, Clone)]"));
    res.extend(parse_ts("pub"));
    res.extend(parse_ts("enum"));
    res.extend(Some(name));

    // res.extend(parse_ts("{"));

    let mut variants = TokenStream::new();
    let mut s = String::from("{");
    for it in deduplicated {
        s.push_str(&it.ident);
        s.push(',');
        variants.extend(parse_ts(&it.ident));
        variants.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))))
    }
    s.push_str("__LAST");
    s.push('}');
    res.extend(parse_ts(&s));
    // return Err(format!("res = {}", variants.to_string()));

    // let variants = Group::new(Delimiter::Brace, variants);

    // res.extend(variants.stream());

    // return Err(format!("res = {}", res.to_string()));
    Ok(res)
}

fn parse_ts(s: &str) -> TokenStream {
    s.parse().unwrap()
}
