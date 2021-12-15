#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::{Delimiter, Ident, TokenStream};
use proc_macro2::{Span, TokenTree};
use quote::quote;

mod trie;

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
                         let endpoint: Option<Keyword> = if i == #keyword.len() - 1 {
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
pub fn make_keywords(attr: TokenStream, item: TokenStream) -> TokenStream {
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
        use crate::trie;
        use lazy_static::lazy_static;
        lazy_static! {
            static ref KEYWORD_TRIE: trie::TrieNode::<#enum_type> = {
                let mut root_node = trie::TrieNode::<#enum_type>::new();
                #new_tree
                root_node
            };
        }
        #new_item
    };
    TokenStream::from(new_tree)
}
