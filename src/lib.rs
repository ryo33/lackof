use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, visit_mut::VisitMut};

#[proc_macro_attribute]
pub fn maybe_unimplementable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemTrait);
    let cloned = process(&item);
    quote! {
        #item
        #cloned
    }
    .into()
}

fn process(item: &syn::ItemTrait) -> syn::ItemTrait {
    let mut cloned = item.clone();
    cloned.ident = format_ident!("MaybeUnimplemented{}", item.ident);
    Unimplementizer.visit_item_trait_mut(&mut cloned);
    cloned
}

struct Unimplementizer;

impl VisitMut for Unimplementizer {
    fn visit_trait_item_fn_mut(&mut self, i: &mut syn::TraitItemFn) {
        i.default = Some(i.default.clone().unwrap_or_else(|| {
            parse_quote! {
                {
                    unimplemented!()
                }
            }
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quote::ToTokens;

    #[test]
    fn test() {
        let input = parse_quote! {
            trait Foo {
                fn foo(&self) -> u32;
                fn bar(&self) -> u32;
            }
        };
        let actual = process(&input).into_token_stream();
        let expected = quote! {
            trait MaybeUnimplementedFoo {
                fn foo(&self) -> u32 {
                    unimplemented!()
                }
                fn bar(&self) -> u32 {
                    unimplemented!()
                }
            }
        };
        assert_eq!(expected.to_string(), actual.to_string());
    }

    #[test]
    fn test_already_has_default() {
        let input = parse_quote! {
            trait Foo {
                fn foo(&self) -> u32;
                fn bar(&self) -> u32 {
                    42
                }
            }
        };
        let actual = process(&input).into_token_stream();
        let expected = quote! {
            trait MaybeUnimplementedFoo {
                fn foo(&self) -> u32 {
                    unimplemented!()
                }
                fn bar(&self) -> u32 {
                    42
                }
            }
        };
        assert_eq!(expected.to_string(), actual.to_string());
    }
}
