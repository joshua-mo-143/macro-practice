use proc_macro::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    Data, DeriveInput, Ident, ItemFn,
    token,
    parenthesized,
    LitInt
};

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

#[proc_macro_attribute]
pub fn mojibake(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);

    let num = args.nr;

    let input = syn::parse2::<DeriveInput>(input.into())
        .expect("Couldn't parse into DeriveInput, are you using this on a Struct?");

    let struct_name = input.ident;

    let span = Span::mixed_site().into();

    let data = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            panic!("This isn't a struct!");
        }
    };

    let operator = if args.up_or_down == "up" {
        quote! {+}
    } else {
        quote! {-}
    };

    let fields = data.fields.clone();

    let field_names: Vec<_> = data
        .fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect();

    let field_names_to_mojibake = field_names.iter().map(|ref x| {
        Ident::new(
            format!("{}_to_mojibake", x).as_str(),
            Span::call_site().into(),
        )
    });

    let code = quote_spanned! {span=>
        struct #struct_name
            #fields

        impl #struct_name {
            #(
                fn #field_names_to_mojibake(&self) -> String {
                    let str = &self.#field_names.to_string();
                    let (res, encoding, had_errors) = SHIFT_JIS.encode(&str);

                    let vec: Vec<u8> = res.into_owned().to_vec().iter().map(|x| 
                        x #operator #num
                    ).collect();

                    let (data, encoding, had_errors) = SHIFT_JIS.decode(&vec);

                    data.into_owned().to_string()
                }
            )*
        }
    };

    TokenStream::from(code)
}

#[allow(dead_code)]
struct Args {
    encode_type: Ident,
    comma: token::Comma,
    up_or_down: Ident,
    paren: token::Paren,
    nr: LitInt
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Args {
            encode_type: input.parse()?,
            comma: input.parse()?,
            up_or_down: input.parse()?,
            paren: parenthesized!(content in input),
            nr: content.parse()?
            })
    }
}