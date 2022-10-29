extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn create_entry(items: TokenStream) -> TokenStream {
    let mut iter = proc_macro2::TokenStream::from(items).into_iter();
    let enum_type = match iter.nth(0).expect("Expected an enum type in macro!") {
        TokenTree::Ident(i) => i,
        _ => panic!("Potato"),
    };
    let f = iter.nth(2);

    match f {
        Some(o) => match o {
            proc_macro2::TokenTree::Ident(i) => {
                let keyword = i.to_string().to_lowercase();
                let code = quote! {
                    {
                        let mut current_node = &mut root_node;
                     for (i, c) in #keyword.as_bytes().iter().enumerate() {
                         let endpoint: Option<#enum_type> = if i == #keyword.len() - 1 {
                             Some(#enum_type::#i)
                         } else {
                             None
                         };
                        // #enum_type;
                        current_node = current_node.insert(*c, endpoint ).as_mut();
                     }
                    }
                };
                return TokenStream::from(code);
            }
            _ => (),
        },
        None => (),
    }
    TokenStream::default()
}

#[proc_macro_attribute]
pub fn make_keywords(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree = proc_macro2::TokenStream::from(item);
    let new_item = tree.clone();
    let mut new_tree = quote! {};

    let mut iter = tree.into_iter();
    let mut enum_type = TokenTree::Ident(proc_macro2::Ident::new("not", Span::call_site()));
    while let Some(e) = iter.next() {
        match &e {
            TokenTree::Ident(i) => {
                if i.to_string() == "enum" {
                    enum_type = iter.next().unwrap();
                    break;
                }
            }
            _ => (),
        }
    }
    let body = iter.next().expect("Need enum body!");
    match &body {
        TokenTree::Group(g) => {
            let stream = g.stream();
            let mut iter = stream.into_iter();

            while let Some(keyword) = iter.next() {
                match keyword {
                    TokenTree::Ident(i) => {
                        new_tree = quote! {
                            #new_tree
                            create_entry!(#enum_type::#i);
                        };
                    }
                    _ => {}
                }
            }
        }
        _ => (),
    }
    new_tree = quote! {
        use lazy_static::lazy_static;
        lazy_static! {
            static ref KEYWORD_TRIE: ts_trie::TrieNode::<#enum_type> = {
                let mut root_node = ts_trie::TrieNode::<#enum_type>::new();
                #new_tree
                root_node
            };
        }
        #new_item
    };
    TokenStream::from(new_tree)
}

#[proc_macro_derive(KeywordFromSlice)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    // let tree = proc_macro2::TokenStream::from(item);
    // let iter = tree.into_iter();

    let input = parse_macro_input!(item as DeriveInput);

    match input.data {
        syn::Data::Enum(e) => {
            let mut st = "".to_string();

            for varient in &e.variants {
                if varient.fields.is_empty() {
                    if varient.ident == "__Unknown" {
                        st.push_str(
                            format!(
                                "            _ => {},\n",
                                varient.ident
                            )
                            .as_str(),
                        );
                    } else {
                        st.push_str(
                            format!(
                                "            \"{}\" => {},\n",
                                varient.ident.to_string().to_lowercase(),
                                varient.ident
                            )
                            .as_str(),
                        );
                    }
                }
            }

            let mut st = format!(
                "impl From<&str> for {} {{\n    fn from(content: &str) -> Self {{\n        use {}::*;\n        match content {{\n{}        }}\n    }}\n}}",
                input.ident,
                input.ident,
                st
            );
            return st.parse().unwrap();
        }
        _ => (),
    }

    TokenStream::new()
}
