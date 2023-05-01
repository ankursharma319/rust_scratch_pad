
// proc_macro comes with rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Cow)]
pub fn cow_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_cow_macro(&ast)
}

fn impl_cow_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Cow for #name {
            fn moo(&self) -> String {
                return format!("Moo from {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
