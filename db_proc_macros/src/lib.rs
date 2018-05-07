#![feature(proc_macro)]
#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


/// CRD stands for Create, Read, Delete.
/// This macro takes a given type and derives methods that allow it to be
/// inserted, read from, and deleted from a database.
#[proc_macro_derive(Crd, attributes(table_name, insertable))]
pub fn crd(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_crd(&ast);
    // Return the generated impl
    gen.into()
}

fn impl_crd( ast: &syn::DeriveInput) -> quote::Tokens {
    let name: &syn::Ident = &ast.ident;

    let table_name: syn::Ident = get_value_from_attributes("table_name", &ast.attrs);
    let insertable: syn::Ident = get_value_from_attributes("insertable", &ast.attrs);

    quote! {

        use db::CRD as macro_CRD;
        use db::Retrievable as macro_Retrievable;
        use db::Creatable as macro_Creatable;
        use db::Deletable as macro_Deletable;
        use error::JoeResult as macro_JoeResult;

        impl macro_Creatable<#insertable> for #name {
            fn create(insert: #insertable, conn: &Conn) -> macro_JoeResult<Self> {
                use schema::#table_name;
                use diesel;
                use diesel::RunQueryDsl;

                diesel::insert_into(#table_name ::table)
                    .values(&insert)
                    .get_result(conn.deref())
                    .map_err(#name::handle_error)
            }

        }

        impl<'a> macro_Retrievable<'a> for #name {
            fn get_by_id(item_id: i32, conn: &Conn) -> macro_JoeResult<#name> {
                use schema::#table_name::dsl::*;
                use diesel::RunQueryDsl;
                use diesel::QueryDsl;

                #table_name
                    .find(item_id)
                    .first::<#name>(conn.deref())
                    .map_err(#name::handle_error)
            }

            fn get_all(conn: &Conn) -> macro_JoeResult<Vec<#name>> {
                use schema::#table_name::dsl::*;
                use diesel::RunQueryDsl;
                #table_name
                    .load::<#name>(conn.deref())
                    .map_err(#name::handle_error)
            }

            fn exists(item_id: i32, conn: &Conn) -> macro_JoeResult<bool> {
                use schema::#table_name;
                use schema::#table_name::dsl::*;
                use diesel::select;
                use diesel::dsl::exists;
                use diesel::RunQueryDsl;
                use diesel::QueryDsl;
                use diesel::ExpressionMethods;

                select(exists(#table_name.filter(#table_name::id.eq(item_id))))
                    .get_result::<bool>(conn.deref())
                    .map_err(#name::handle_error)
            }
            // fn get_paginated(page_index: i64, page_size: i64, conn: &Conn) -> Result<Vec<#name>, WeekendAtJoesError> {
            //     use schema::#table_name::dsl::*;
            //     use diesel::associations::HasTable;
            //     use diesel::RunQueryDsl;
            //     use diesel::prelude::*;
            //     use db::diesel_extensions::pagination::Paginate;

            //     #table_name ::tab
            //         .paginate(page_index)
            //         .per_page(page_size)
            //         .load_and_count_pages(conn.deref());

            //     unimplemented!()
            // }

        }

        impl<'a> macro_Deletable<'a> for #name {
            fn delete_by_id(item_id: i32, conn: &Conn) -> macro_JoeResult<#name> {
                use schema::#table_name::dsl::*;
                use diesel::ExpressionMethods;
                use diesel;
                use diesel::RunQueryDsl;
                use diesel::QueryDsl;

                let target = #table_name.filter(id.eq(item_id));

                diesel::delete(target)
                    .get_result(conn.deref())
                    .map_err(#name::handle_error)
            }
        }


        impl<'a> macro_CRD<'a, #insertable> for #name {}
    }
}

/// Derive a macro that implements the error handling for DB types.
#[proc_macro_derive(ErrorHandler)]
pub fn error_handler(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_error_handler(&ast);
    // Return the generated impl
    gen.into()
}

fn impl_error_handler( ast: &syn::DeriveInput) -> quote::Tokens {
    let name: &syn::Ident = &ast.ident;

    // Convert the name of the class into a string, so that it may be used
    // to identify what DB method went wrong
    quote!(

        use error::handle_diesel_error as macro_handle_diesel_error;
        use error::WeekendAtJoesError as macro_WeekendAtJoesError;
        use error::ErrorFormatter;
        use diesel::result::Error as macro_Error;

        impl ErrorFormatter for #name {
            fn handle_error(diesel_error: macro_Error) -> macro_WeekendAtJoesError {
                macro_handle_diesel_error(diesel_error, stringify!(#name))
            }
        }

    )
}

/// Given a string that coreesponds to the ident specified in the attributes section in the proc_macro_derive(...) above
/// extract the value that corresponds to it.
fn get_value_from_attributes(attribute_name: &'static str, attrs: &Vec<syn::Attribute>) -> syn::Ident {
    attrs
        .into_iter()
        .map(|x| x.interpret_meta(). unwrap())
        .filter(|x| {

            if let syn::Meta::NameValue(ref t) = *x {
                // We found our attribute, now extract the value in the next map.
                if t.ident.as_ref() == attribute_name {
                    return true
                }
                else {
                    false
                }
            } else {
                false // the attribute wasn't in the proper form: "thing" = value
            }
        })
        .map(|x| {
            if let syn::Meta::NameValue(t) = x {
                if let syn::Lit::Str(l) = t.lit {
                    let ident = syn::Ident::from(l.value());
                    return ident
                }
                panic!("Couldn't extract value from attribute")
            }
            panic!("Wasn't namevalue")
        })
        .collect::<Vec<syn::Ident>>()
        .first()
        .expect(format!("The attribute '{}' could not be found", attribute_name).as_ref())
        .clone()
}