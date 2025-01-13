extern crate proc_macro2;

use proc_macro::TokenStream;
use crate::echoes::EchoDataItem;
use quote::quote;

mod echoes;

fn get_echo_names(data: &[EchoDataItem]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();

    for item in data.iter() {
        result.push(item.name.parse().unwrap());
    }

    result
}

fn echo_name_from_chs(data: &[EchoDataItem], echo_names: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    let chs_names: Vec<_> = data.iter().map(|x| x.name_chs.clone()).collect();

    let mut temp = Vec::new();
    for i in 0..echo_names.len() {
        let name = &chs_names[i];
        let echo_name = &echo_names[i];
        temp.push(quote! {
            #name => Some(Self:: #echo_name),
        });
    }

    quote! {
        impl WWEchoName {
            pub fn from_chs(chs: &str) -> Option<Self> {
                match chs {
                    #(#temp)*
                    // It's weird that this will not compile
                    // #(#chs_names => Some(Self::#echo_names)),*
                    _ => return None,
                }
            }
        }
    }
}

#[proc_macro]
pub fn yas_wuthering_waves_echoes(input: TokenStream) -> TokenStream {
    let ast: syn::LitStr = syn::parse(input).unwrap();

    let filename = ast.value();

    let content = std::fs::read_to_string(filename).unwrap();
    let echo_data: Vec<EchoDataItem> = serde_json::from_str(&content).unwrap();

    let echo_names = get_echo_names(&echo_data);

    let echo_name_enum = quote! {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, strum_macros::Display)]
        pub enum WWEchoName {
            #(#echo_names),*
        }
    };
    let echo_name_from_chs_impl = echo_name_from_chs(&echo_data, &echo_names);

    let result = quote! {
        #echo_name_enum
        #echo_name_from_chs_impl
    };

    // println!("{:?}", result.to_string());

    result.into()
}
