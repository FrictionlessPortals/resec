//! A collection of proc-macros for resec.
//! 
//! These proc-macros allow the library to generate robust schema's
//! from simple syntax.

use proc_macro::TokenStream;
use std::{fs, collections::BTreeMap};
use quote::{quote, format_ident};
use syn::{parse_macro_input, LitStr};
use proc_macro2::TokenStream as TokenStream2;
use regex::Regex;

#[proc_macro]
pub fn make_schema(input: TokenStream) -> TokenStream {    
    // Parse the input tokens into a syntax tree
    let input: LitStr = parse_macro_input!(input);

    // Load the schema file.
    let contents = fs::read_to_string(input.value()).unwrap();
    let map: BTreeMap<u32, String> = serde_json::from_str(&contents).unwrap();

    // Compile the regex to reduce function runtime.
    let regex = Regex::new("[^0-9a-zA-Z]+").unwrap();

    // Form tokenstream from the given map.
    let output: Vec<TokenStream2> = map.into_iter().map(|(k, v)| {
        // Format the name.
        let formatted_name = v.replace(' ', "");

        // Replace the characters using regex.
        let replaced = regex.replace_all(&formatted_name, "_");

        // Generate idents.
        let ident_name = format_ident!("{}", replaced.to_string());

        // Generate the field.
        let token = quote! {
            #[strum(props(name = #ident_name, id = #k))]
            #ident_name,
        };

        token
    }).collect();

    // Form the enum.
    let tokens = quote! {
        /// Subjects that offer documents on the SEC website.
        /// Each subject contains its name and id that can be used to generate a query.
        #[derive(EnumProperty, EnumIter, Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub enum Subject {
            #(#output)*
        }
    };

    TokenStream::from(tokens)
}