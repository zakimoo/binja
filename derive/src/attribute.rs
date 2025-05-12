use darling::FromField;

#[derive(Debug, FromField)]
#[darling(attributes(binja))]
pub struct BinjaFieldOpts {
    #[allow(unused)]
    pub ident: Option<syn::Ident>,

    // #[binja(skip)]
    pub skip: Option<()>,
}
