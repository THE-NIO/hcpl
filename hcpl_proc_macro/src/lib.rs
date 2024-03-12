/*!
This crate contains procedural macros for `hcpl`.
*/

mod gen;
mod result;

use gen::{gen_read_from_fields, gen_read_from_union, gen_read_from_variants};
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error, set_dummy};
use quote::quote;
use result::ErrorKind;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DataEnum, DataStruct, DataUnion,
    DeriveInput, Fields, FieldsNamed, Generics, Ident, Token, Variant,
};

/**
Derive macro generating an impl of the trait
<a href="../hcpl_io/trait.Cinable.html">`Cinable`</a>.
You can find documentation on how to use this
derive macro in  <a href="../hcpl_io/index.html">
the documentation of `hcpl_io`</a>.
*/
#[proc_macro_derive(Cinable, attributes(tag_type, tag))]
#[proc_macro_error]
pub fn derive_cinable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    set_dummy(quote! {
        impl hcpl_io::Cinable for #item_name {
            fn read_from(cin: &mut hcpl_io::Cin) -> Self {
                unimplemented!();
            }
        }
    });

    let ts = match &item.data {
        Data::Struct(DataStruct { fields, .. }) => {
            derive_cinable_for_struct(item_name, fields, &item.generics)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            derive_cinable_for_enum(item_name, &item.attrs, variants, &item.generics)
        }
        Data::Union(DataUnion { fields, .. }) => {
            derive_cinable_for_union(item_name, &item.attrs, fields, &item.generics)
        }
    };

    match ts {
        Ok(ts) => ts.into(),
        Err(ErrorKind::SpanLater(msg)) => abort!(item, msg),
        Err(ErrorKind::Spanned(err)) => err.into_compile_error().into(),
    }
}

fn derive_cinable_for_struct(
    name: &Ident,
    fields: &Fields,
    generics: &Generics,
) -> result::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let read_from_fields = gen_read_from_fields(fields, name)?;

    let ts = quote! {
        #[automatically_derived]
        impl #impl_generics hcpl_io::Cinable for #name #ty_generics #where_clause {
            fn read_from(cin: &mut hcpl_io::Cin) -> Self {
                #read_from_fields
            }
        }
    };
    Ok(ts)
}

fn derive_cinable_for_enum(
    name: &Ident,
    attrs: &Vec<Attribute>,
    variants: &Punctuated<Variant, Token![,]>,
    generics: &Generics,
) -> result::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let read_from_variants = gen_read_from_variants(variants, attrs)?;

    let ts = quote! {
        #[automatically_derived]
        impl #impl_generics hcpl_io::Cinable for #name #ty_generics #where_clause {
            fn read_from(cin: &mut hcpl_io::Cin) -> Self {
                #read_from_variants
            }
        }
    };
    Ok(ts)
}

fn derive_cinable_for_union(
    name: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    generics: &Generics,
) -> result::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let read_from_union = gen_read_from_union(fields, attrs)?;

    let ts = quote! {
        #[automatically_derived]
        impl #impl_generics hcpl_io::Cinable for #name #ty_generics #where_clause {
            fn read_from(cin: &mut hcpl_io::Cin) -> Self {
                #read_from_union
            }
        }
    };
    Ok(ts)
}
