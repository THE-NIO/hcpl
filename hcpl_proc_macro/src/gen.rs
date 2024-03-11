use crate::result::{self, ErrorKind};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, Attribute, Error, Expr, ExprLit, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Meta, MetaNameValue, Pat, Token, Type, Variant,
};

pub(crate) fn gen_read_from_fields(fields: &Fields, ident: &Ident) -> result::Result<TokenStream> {
    let ts = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields = named.iter().map(|field| {
                let fident = field.ident.as_ref().unwrap();
                quote!(#fident: cin.get())
            });
            quote! ( #ident {#(#fields),*} )
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let fields = unnamed.iter().map(|_| quote!(cin.get()));

            quote! { #ident (#(#fields),*) }
        }
        Fields::Unit => quote!(#ident),
    };
    Ok(ts)
}

pub(crate) fn gen_read_from_variants(
    variants: &Punctuated<Variant, Token![,]>,
    attrs: &Vec<Attribute>,
) -> result::Result<TokenStream> {
    let ty = get_tag_type(attrs)?;

    let mut match_arms = vec![];
    for variant in variants {
        let pat = match get_tag_pat(variant) {
            Ok(pat) => pat,
            Err(ek) => return ek.span_now(variant).into(),
        };
        let read_from_fields = gen_read_from_fields(&variant.fields, &variant.ident)?;
        match_arms.push(quote!( #pat => Self::#read_from_fields ));
    }
    match_arms.push(quote!( _ => panic!("unexpected value: {:?}", tag) ));

    let ts = quote! {
        let tag = cin.get::<#ty>();
        match tag {
            #(#match_arms),*
        }
    };
    Ok(ts)
}

pub(crate) fn gen_read_from_union(
    fields_named: &FieldsNamed,
    attrs: &Vec<Attribute>,
) -> result::Result<TokenStream> {
    let ty = get_tag_type(attrs)?;
    let FieldsNamed {
        named: fields_named,
        ..
    } = fields_named;

    let mut match_arms = vec![];
    for field_named in fields_named {
        let pat = match get_tag_pat(field_named) {
            Ok(pat) => pat,
            Err(ek) => return ek.span_now(field_named).into(),
        };
        let fident = field_named.ident.as_ref().unwrap();
        match_arms.push(quote!( #pat => Self { #fident: cin.get() } ));
    }
    match_arms.push(quote!( _ => panic!("unexpected value: {:?}", tag) ));

    let ts = quote! {
        let tag = cin.get::<#ty>();
        match tag {
            #(#match_arms),*
        }
    };
    Ok(ts)
}

fn get_tag_type(attrs: &Vec<Attribute>) -> result::Result<Type> {
    let tag_type_attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("tag_type"))
        .collect();

    let tag_type_attr = if tag_type_attrs.len() == 1 {
        tag_type_attrs[0]
    } else {
        const MESSAGE: &str =
            "there must be exactly one tag_type attribute when deriving Cinable for enum";
        return if tag_type_attrs.is_empty() {
            ErrorKind::SpanLater(MESSAGE.to_owned()).into()
        } else {
            Err(Error::new_spanned(tag_type_attrs[1], MESSAGE).into())
        };
    };

    let meta_list = match tag_type_attr.meta.require_list() {
        Ok(meta_list) => meta_list,
        Err(_) => abort!(
            tag_type_attr,
            "expected attribute list (e.g. #[tag_type(bool)])"
        ),
    };

    let ty = match meta_list.parse_args::<Type>() {
        Ok(ty) => ty,
        Err(_) => abort!(meta_list.delimiter.span().join(), "expected a single type"),
    };
    Ok(ty)
}

trait Attrs {
    fn attrs(&self) -> &Vec<Attribute>;
}

impl Attrs for syn::Field {
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}

impl Attrs for syn::Variant {
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}

fn get_tag_pat<T: Attrs>(item: &T) -> result::Result<Punctuated<Pat, Token![|]>> {
    let tag_attrs: Vec<_> = item
        .attrs()
        .iter()
        .filter(|attr| attr.path().is_ident("tag"))
        .collect();

    if tag_attrs.is_empty() {
        return ErrorKind::SpanLater(
            "there must be at least one tag attribute for each enum variant".to_owned(),
        )
        .into();
    }

    let mut or_pats = Punctuated::<Pat, Token![|]>::new();
    for tag_attr in tag_attrs {
        match &tag_attr.meta {
            Meta::NameValue(MetaNameValue { value, .. }) => match value {
                Expr::Lit(expr_lit) => or_pats.push(expr_lit.clone().into()),
                _ => abort!(value, "attribute value must be a literal"),
            },
            Meta::List(meta_list) => {
                if meta_list.tokens.is_empty() {
                    abort!(
                        meta_list.delimiter.span().join(),
                        "expected at least one literal"
                    );
                }

                let expr_lits: Punctuated<ExprLit, Token![,]> =
                    Parser::parse2(Punctuated::parse_terminated, meta_list.tokens.clone())?;

                or_pats.extend(
                    expr_lits
                        .iter()
                        .map(|expr_lit| -> Pat { expr_lit.clone().into() }),
                )
            }
            Meta::Path(path) => {
                abort!(
                    path,
                    "path-only syntax unsupported.\n\
                    please use name-value syntax (#[tag = '1']) or list-style syntax (#[tag(3, 5)])"
                )
            }
        };
    }
    Ok(or_pats)
}
