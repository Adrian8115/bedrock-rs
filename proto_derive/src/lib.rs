use de::proto_build_de_struct;
use quote::quote;
use ser::proto_build_ser_struct;
use syn::{parse_macro_input, Data, DeriveInput};

mod de;
mod ser;

#[proc_macro_derive(ProtoCodec, attributes(len_type, enum_repr))]
pub fn proto_codec(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = match input.data {
        Data::Struct(struct_data) => {
            let ser = proto_build_ser_struct(&struct_data);
            let de = proto_build_de_struct(&struct_data);

            quote! {
                impl #impl_generics proto_core::ProtoCodec for #name #ty_generics #where_clause {
                    fn proto_serialize(&self, stream: &mut bedrock_core::stream::write::ByteStreamWrite) -> Result<(), proto_core::error::ProtoCodecError> where Self: Sized {
                        #ser
                        Ok(())
                    }

                    fn proto_deserialize(stream: &mut bedrock_core::stream::read::ByteStreamRead) -> Result<Self, proto_core::error::ProtoCodecError> where Self: Sized {
                        Ok(Self{
                            #de
                        })
                    }
                }
            }
        }
        Data::Enum(_) => {
            // Unions are not supported
            panic!("ProtoCodec macro only supports named/unnamed structs, got enum: {name:?}.")
        }
        Data::Union(_) => {
            // Unions are not supported
            panic!("ProtoCodec macro only supports named/unnamed structs, got union: {name:?}.")
        }
    };

    proc_macro::TokenStream::from(expanded)
}
