#![feature(proc_macro)]
#![recursion_limit="128"]
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

        impl macro_Creatable<#insertable> for #name {
            fn create(insert: #insertable, conn: &Conn) -> Result<Self, WeekendAtJoesError> {
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
            fn get_by_id(item_id: i32, conn: &Conn) -> Result<#name, WeekendAtJoesError> {
                use schema::#table_name::dsl::*;
                use diesel::RunQueryDsl;
                use diesel::QueryDsl;

                #table_name
                    .find(item_id)
                    .first::<#name>(conn.deref())
                    .map_err(#name::handle_error)
            }
        }

        impl<'a> macro_Deletable<'a> for #name {
            fn delete_by_id(item_id: i32, conn: &Conn) -> Result<#name, WeekendAtJoesError> {
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