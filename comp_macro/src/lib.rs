use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comp);

    quote! {#c}.into()
}

struct Comp {
    mapping: Mapping,
    for_if_clauses: Vec<ForIfClause>,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clauses: parse_one_or_more(input)?,
        })
    }
}

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            mapping,
            for_if_clauses,
        } = self;
        let mut previous = quote! {
            Some(#mapping)
        };
        for ForIfClause {
            pattern,
            iter,
            clauses,
        } in for_if_clauses
        {
            previous = quote! {
                ::core::iter::IntoIterator::into_iter(#iter).flat_map(move |#pattern| {
                    ::core::iter::IntoIterator::into_iter((true #(&& (#clauses))*).then(|| #previous)).flatten()
                })
            }
        }
        tokens.extend(previous);
    }
}

fn parse_one_or_more<T: Parse>(input: ParseStream) -> syn::Result<Vec<T>> {
    Ok(parse_more(vec![input.parse()?], input))
}

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    iter: syn::Expr,
    clauses: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pattern = input.parse()?;
        input.parse::<Token![in]>()?;
        let iter = input.parse()?;
        let clauses = parse_zero_or_more(input);

        Ok(ForIfClause {
            pattern,
            iter,
            clauses,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    parse_more(Vec::new(), input)
}

fn parse_more<T: Parse>(mut result: Vec<T>, input: ParseStream) -> Vec<T> {
    while let Ok(v) = input.parse() {
        result.push(v)
    }
    result
}

struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}
