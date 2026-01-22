use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    visit_mut::{self, VisitMut},
    Expr, ExprMethodCall, ExprPath, ItemFn,
};
#[proc_macro_attribute]
pub fn client(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #[allow(dead_code)]
        #input
    };
    TokenStream::from(expanded)
}
struct OnClickVisitor;
impl VisitMut for OnClickVisitor {
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        visit_mut::visit_expr_method_call_mut(self, node);
        if node.method == "on_click" && node.args.len() == 1 {
            if let Expr::Path(ExprPath { path, .. }) = &node.args[0] {
                let path_str = path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("_");
                let final_str = format!("Lithe.dispatch('{}')", path_str);
                let new_arg: Expr = syn::parse_quote!(#final_str);
                node.args[0] = new_arg;
            }
        }
    }
}
#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let mut visitor = OnClickVisitor;
    visitor.visit_item_fn_mut(&mut input);
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}
