use proc_macro2::TokenStream;

use quote::{format_ident, quote};

use syn::{DataStruct, Result};
use syn::{Fields, Ident};

pub(crate) fn parse_struct(type_ident: Ident, data: DataStruct) -> Result<TokenStream> {
    let fields_size = data.fields.len();

    let fields = if let Fields::Named(fields) = data.fields {
        fields
            .named
            .into_iter()
            .map(|f| f.ident.unwrap())
            .collect::<Vec<Ident>>()
    } else {
        return Err(syn::Error::new_spanned(
            type_ident,
            "ListSerialize can only be used on structs with named fields",
        ))?;
    };
    let visitor_name = format_ident!("{}Visitor", type_ident);
    let visitor_in_place_name = format_ident!("{}InPlaceVisitor", type_ident);
    Ok(quote! {
        impl serde::Serialize for #type_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer{
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(#fields_size))?;
                #(
                    seq.serialize_element(&self.#fields)?;
                )*
                seq.end()
            }
        }
        pub struct #visitor_name;
        impl<'de> serde::de::Visitor<'de> for #visitor_name {
            type Value = #type_ident;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("a sequence of ", #fields_size, " elements"))
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                use serde::de::Error;
                if let Some(size) = seq.size_hint() {
                    if size != #fields_size {
                        return Err(serde::de::Error::invalid_length(size, &self));
                    }
                }
                Ok(#type_ident {
                    #(
                        #fields: seq.next_element()?.ok_or_else(|| A::Error::invalid_length(#fields_size, &self))?,
                    )*
                })
            }
        }
        pub struct #visitor_in_place_name<'data>(&'data mut #type_ident);

        impl<'de,'data> serde::de::Visitor<'de> for #visitor_in_place_name<'data> {
            type Value = ();
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("a sequence of ", #fields_size, " elements"))
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                use serde::de::Error;
                if let Some(size) = seq.size_hint() {
                    if size != #fields_size {
                        return Err(serde::de::Error::invalid_length(size, &self));
                    }
                }
                #(
                    self.0.#fields = seq.next_element()?.ok_or_else(|| A::Error::invalid_length(#fields_size, &self))?;
                )*
                Ok(())
            }
        }


        impl<'de> serde::Deserialize<'de> for #type_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                deserializer.deserialize_seq(#visitor_name)
            }
            fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: serde::Deserializer<'de> {
                deserializer.deserialize_seq(#visitor_in_place_name(place))
            }
        }

    })
}
