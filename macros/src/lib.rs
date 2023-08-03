use proc_macro::{TokenStream, Span};
use syn::{ItemFn, Ident};
use quote::{quote_spanned};

#[proc_macro_attribute]
pub fn benchmark(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse2::<ItemFn>(input.into()).unwrap();

    let name = format!("{}", input.sig.ident);

    let name_as_ident = Ident::new(&name, Span::call_site().into());
    let code = input.block;

    let span = Span::mixed_site().into();

    let code = quote_spanned! {span=>
        fn #name_as_ident() {
        let time = std::time::Instant::now();
            
        println!("The name of this function is: {}", #name);
            #code
        println!("Amount of time taken for this fn to complete: {:?}", time.elapsed());
        }

    };

    TokenStream::from(code)
}