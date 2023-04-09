use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ExtraOps)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_trait(&ast)
}

fn impl_my_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics ExtraOptions for #name #ty_generics {
            fn modify_element(&mut self, options: Vec<Options>){
                self.latex = self.to_latex_string();
                for option in options {
                    self.latex = option.modify(&self.latex);
                }
                self.modified = true;
            }
        }
    };
    gen.into()
}
