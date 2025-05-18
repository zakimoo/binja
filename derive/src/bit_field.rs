use quote::quote;

pub const VALID_BIT_FIELD_TYPES: [&str; 13] = [
    "bool", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
];

pub fn is_valid_bit_field_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if VALID_BIT_FIELD_TYPES.contains(&segment.ident.to_string().as_str()) {
                return true;
            }
        }
    }
    false
}

pub fn gen_bit_field_serialization(
    code: &mut Vec<proc_macro2::TokenStream>,
    field_expr: &proc_macro2::TokenStream,
    bits: u8,
    bit_offset: &mut u8,
    allow_overflow: bool,
) {
    let mut bits_remaining = bits;
    let mut local_shift = 0u8;

    if !allow_overflow {
        // check if the field is too big
        code.push(quote! {
            if( #field_expr >> #bits) != 0 {
                return Err(::binja::error::Error::Overflow{
                    value: format!("{:#x}", #field_expr),
                    max: format!("{:#x}", binja::bit_mask!(#bits) ),
                });
            }
        });
    }

    while bits_remaining > 0 {
        let byte_bit_pos = *bit_offset % 8;
        let bits_in_current_byte = 8 - byte_bit_pos;
        let bits_to_write = bits_remaining.min(bits_in_current_byte);

        let mask = quote! { ::binja::bit_mask!(#bits_to_write)  };

        let value_expr = if local_shift == 0 {
            quote! { (#field_expr & #mask) }
        } else {
            quote! { ((#field_expr >> #local_shift) & #mask) }
        };

        code.push(quote! {
            bit_field |= ((#value_expr) as u8) << #byte_bit_pos;
        });

        *bit_offset += bits_to_write;
        local_shift += bits_to_write;
        bits_remaining -= bits_to_write;

        // if we have written 8 bits, flush the bit field
        if *bit_offset % 8 == 0 {
            code.push(quote! {
                // flush bit field
                ::binja::ser::binary_serialize(&bit_field, serializer)?;
                // Reset bit_field to 0 after serialization
                bit_field = 0u8;
            });
        }
    }
}

pub fn flush_bit_field_if_needed(code: &mut Vec<proc_macro2::TokenStream>, bit_offset: &mut u8) {
    // if last field is a bit field smaller that 8 bits
    // current field is not a bit field
    if *bit_offset % 8 != 0 {
        code.push(quote! {
            // flush bit field
            ::binja::ser::binary_serialize(&bit_field, serializer)?;
            // Reset bit_field to 0 after serialization
            bit_field = 0u8;
        });
        // align to next byte
        *bit_offset = (*bit_offset).div_ceil(8) * 8;
    }
}

pub fn flush_bit_field_at_end(code: &mut Vec<proc_macro2::TokenStream>, bit_offset: u8) {
    // if last field is a bit field smaller that 8 bits
    if bit_offset % 8 != 0 {
        // flush bit field
        code.push(quote! {
            ::binja::ser::binary_serialize(&bit_field, serializer)?;
        });
    }
}
