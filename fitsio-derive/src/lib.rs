use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(FitsRow, attributes(fitsio))]
pub fn read_row(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let mut tokens = Vec::new();

    match input.data {
        syn::Data::Struct(ref s) => match s.fields {
            syn::Fields::Named(ref fields) => {
                for field in &fields.named {
                    let ident = &field.ident.as_ref().unwrap();
                    let ident_str = ident.to_string();
                    if field.attrs.is_empty() {
                        let src = quote::quote! {
                            out.#ident = tbl.read_cell_value(fits_file, #ident_str, idx)?;
                        };
                        tokens.push(src);
                    } else {
                        for attr in &field.attrs {
                            match attr.parse_meta() {
                                Ok(syn::Meta::List(l)) => {
                                    for entry in l.nested {
                                        match entry {
                                            syn::NestedMeta::Meta(syn::Meta::NameValue(
                                                syn::MetaNameValue { path, lit, .. },
                                            )) => {
                                                if !path.is_ident("colname") {
                                                    continue;
                                                }

                                                match lit {
                                                    syn::Lit::Str(ls) => {
                                                        tokens.push(quote::quote! {
                                                            out.#ident = tbl.read_cell_value(
                                                                fits_file,
                                                                #ls,
                                                                idx)?;
                                                        });
                                                    }
                                                    _ => panic!(
                                                "Only #[fitsio(colname = \"...\")] is supported"
                                            ),
                                                }
                                            }
                                            _ => panic!(
                                                "Only #[fitsio(colname = \"...\")] is supported"
                                            ),
                                        }
                                    }
                                }
                                _ => panic!("Only #[fitsio(colname = \"...\")] is supported"),
                            }
                        }
                    }
                }
            }
            _ => panic!("Only #[fitsio(colname = \"...\")] is supported"),
        },
        _ => panic!("derive only possible for structs"),
    }

    let expanded = quote::quote! {
        impl FitsRow for #name {
            fn from_table(
                tbl: &::fitsio::hdu::FitsHdu,
                fits_file: &mut ::fitsio::FitsFile, idx: usize) ->
                    ::fitsio::errors::Result<Self> where Self: Sized  {
                let mut out = Self::default();

                #(#tokens)*

                Ok(out)
            }
        }
    };
    expanded.into()
}
