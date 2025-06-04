use syn::parse_quote;

pub fn add_trait_bounds(generics: &syn::Generics, trait_path: syn::Path) -> syn::Generics {
    // Clone generics to modify
    let mut generics_with_bounds = generics.clone();
    let where_clause = generics_with_bounds.make_where_clause();

    // Add trait bounds to each type parameter
    for param in generics.type_params() {
        let ident = &param.ident;
        where_clause.predicates.push(parse_quote! {
            #ident: #trait_path
        });
    }

    generics_with_bounds
}
