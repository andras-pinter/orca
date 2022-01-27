use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn interface(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = input.ident.clone();
    let output = quote::quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input

        impl Into<wasm_bindgen::JsValue> for #ident {
            fn into(self) -> wasm_bindgen::JsValue {
                wasm_bindgen::JsValue::from(serde_json::to_string(&self).expect("Invalid JSON"))
            }
        }
    };

    output.into()
}
