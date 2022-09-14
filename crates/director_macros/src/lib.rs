extern crate proc_macro;

use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;

#[derive(Debug, Default)]
struct StateArgs {
    sup: Vec<Ident>,
    sub: Vec<Ident>,
}

impl StateArgs {
    fn parse(attr: TokenStream) -> Self {
        let mut args = Self::default();

        let mut cursor = attr.into_iter();

        let pass_punct = |token: TokenTree, punct| {
            if token.to_string() != punct {
                panic!("{token} is unexpected token.")
            }
        };

        while let Some(item) = cursor.next() {
            if let TokenTree::Ident(ident) = item {
                let ident = ident.to_string();
                let command = ident.as_str();
                match command {
                    "super" | "sub" => {
                        pass_punct(cursor.next().unwrap(), "=");
                        let content = cursor.next().unwrap();
                        match content {
                            TokenTree::Group(block) => {
                                if block.delimiter() != Delimiter::Bracket {
                                    unimplemented!()
                                }
                                block.stream().into_iter().for_each(|item| {
                                    if let TokenTree::Ident(state_name) = item {
                                        match command {
                                            "super" => args.sup.push(state_name),
                                            "sub" => args.sub.push(state_name),
                                            _ => unimplemented!(),
                                        }
                                    }
                                })
                            }
                            TokenTree::Ident(state_name) => match command {
                                "super" => args.sup.push(state_name),
                                "sub" => args.sub.push(state_name),
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
        let mut arr = [&args.sub, &args.sup];
        arr.sort_by_key(|item| item.len());
        let [less_vec, more_vec] = arr;

        let less_vec = less_vec
            .iter()
            .map(|state| state.to_string())
            .collect::<Vec<_>>();

        more_vec.iter().for_each(|state| {
            if less_vec.contains(&state.to_string()) {
                panic!("A state cannot be in super state and sub state at the same time.")
            }
        });

        args
    }
}

#[proc_macro_attribute]
pub fn attribute_state(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_body = syn::parse_macro_input!(item as syn::DeriveInput);
    let name = &struct_body.ident;
    let name_string = name.to_string();
    let state_name: syn::Ident =
        syn::parse_str(&format!("__{}__", name.to_string().to_uppercase())).unwrap();

    let args = StateArgs::parse(attr);

    let pass_state_name = |name: &str, kind: &str| {
        if name == name_string {
            panic!("{kind} state name must not be the same as current state name.")
        }
    };

    let run_subs = args
        .sub
        .iter()
        .map(|sub_state_name| {
            let sub_name = sub_state_name.to_string();
            pass_state_name(&sub_name, "Sub"); // ---- check
            format!("{sub_name}::run(engine);")
        })
        .collect::<String>();

    let run_subs_quote: syn::Stmt = syn::parse_str(&format!("{{ {run_subs} }}")).unwrap();

    let drop_subs = args
        .sub
        .iter()
        .map(|sub_state_name| {
            use heck::ToSnakeCase;
            let sub_name = sub_state_name.to_string();
            format!(
                "{{ Self::lock_sub__{snake_case_name}().get_mut().drop(engine); }}",
                snake_case_name = sub_name.to_snake_case()
            )
        })
        .collect::<String>();

    let drop_subs_quote: syn::Stmt = syn::parse_str(&format!("{{ {drop_subs} }}")).unwrap();

    let get_and_get_mut_sups = args
        .sup
        .iter()
        .map(|sup_state_name| {
            use heck::ToSnakeCase;
            let name = sup_state_name.to_string();
            pass_state_name(&name, "Super"); // ---- check
            format!(
                "
                    #[inline]
                    #[allow(non_snake_case)]
                    pub fn lock_super__{snake_case_name}<'a>() -> director::___::StateController<'a, Engine, {name}> {{
                        {name}::lock()
                    }}
                ",
                snake_case_name = name.to_snake_case()
            )
        })
        .collect::<String>();

    let get_and_get_mut_subs = args
        .sub
        .iter()
        .map(|sub_state_name| {
            use heck::ToSnakeCase;
            let name = sub_state_name.to_string();
            format!(
                "
                    #[inline]
                    #[allow(non_snake_case)]
                    pub fn lock_sub__{snake_case_name}<'a>() -> director::___::StateController<'a, Engine, {name}> {{
                        {name}::lock()
                    }}
                ",
                snake_case_name = name.to_snake_case()
            )
        })
        .collect::<String>();

    let get_and_get_mut_quote: syn::ItemImpl = syn::parse_str(&format!(
        "impl {name} {{ {get_and_get_mut_sups} {get_and_get_mut_subs} }}"
    ))
    .unwrap();

    quote! {
        director::___::lazy_static! {
            static ref #state_name: director::___::StateOrigin<#name> = std::sync::Mutex::new(None);
        }

        #struct_body

        impl #name {


           #[inline]
           pub fn lock<'a>() -> director::___::StateController<'a, Engine, #name> {
               director::___::StateController::new(#state_name.lock().unwrap())
           }

           #[inline]
           pub fn run(engine: &mut Engine) {
               use director::State as _____state_trait_____;
               let mut state = Self::lock();

               if !#name::toggle(engine, state.as_inner().as_ref()) {
                   if state.as_inner().is_some() {
                       state.get_mut().drop(engine);
                       state.set(None);
                   }

                   state.unlock();

                   Self::drop_subs(engine);
                   return;
               }

               if state.as_inner().is_none() {
                   state.set(Some(#name::load(engine)));
               }

               if state.as_inner().is_some() {
                   // my logics
                   state.get_mut().run(engine);
               }

               state.unlock();

               Self::run_subs(engine);
           }

           #[inline]
           fn run_subs(engine: &mut Engine) {
               // ... sub::run(engine); ...
               #run_subs_quote;
           }
           #[inline]
           fn drop_subs(engine: &mut Engine) {
                use director::State as _____state_trait_____;
               // ... { sub_state.get_mut().drop(engine); } ...
               #drop_subs_quote;
           }
        }

        #get_and_get_mut_quote
    }
    .into()
}
