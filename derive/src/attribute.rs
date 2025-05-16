use darling::{FromDeriveInput, FromField, FromVariant};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(binja), supports(struct_any))]
pub struct StructAttributes {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(binja), supports(enum_any))]
pub struct EnumAttributes {
    pub ident: syn::Ident,
    pub generics: syn::Generics,

    // #[binja(repr = u32, untagged)]
    pub repr: Option<String>,
    pub untagged: Option<()>,
    // You can extend with more options as needed
}

impl EnumAttributes {
    pub fn repr(&self) -> String {
        let repr = self.repr.clone().unwrap_or("u32".to_string());
        if matches!(
            repr.as_str(),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
        ) {
            repr
        } else {
            panic!("Unsupported repr type: {}", repr)
        }
    }

    pub fn untagged(&self) -> bool {
        self.untagged.is_some()
    }
}

#[derive(Debug, Default, FromField)]
#[darling(attributes(binja))]
pub struct FieldAttributes {
    #[allow(unused)]
    pub ident: Option<syn::Ident>,

    // #[binja(skip)]
    pub skip: Option<()>,

    pub bits: Option<u8>,

    pub no_overflow: Option<()>,
}

impl FieldAttributes {
    pub fn skip(&self) -> bool {
        self.skip.is_some()
    }

    pub fn bits(&self) -> Option<u8> {
        self.bits
    }

    pub fn no_overflow(&self) -> bool {
        self.no_overflow.is_some()
    }

    pub fn validate(&self, span: proc_macro2::Span) -> syn::Result<()> {
        if self.skip.is_some() && self.bits.is_some() {
            return Err(syn::Error::new(
                span,
                "Field cannot be both `skip` and have `bits = N`",
            ));
        }

        if let Some(bits) = self.bits {
            if bits == 0 || bits > 128 {
                return Err(syn::Error::new(span, "bits must be between 1 and 128"));
            }
        }

        if self.no_overflow.is_some() && self.bits.is_none() {
            return Err(syn::Error::new(
                span,
                "no_overflow can only be used with a bits attribute",
            ));
        }

        Ok(())
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(binja))]
pub struct VariantAttributes {
    #[allow(unused)]
    pub ident: syn::Ident,
}
