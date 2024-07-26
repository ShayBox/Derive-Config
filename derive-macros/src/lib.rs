use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn generate_impl(
    struct_name: &syn::Ident,
    trait_name: &str,
    ext_name: &str,
) -> proc_macro2::TokenStream {
    let format_mod = syn::Ident::new(ext_name, proc_macro2::Span::call_site());
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let to_string_fn = syn::Ident::new(
        if ext_name == "yaml" {
            "to_string"
        } else {
            "to_string_pretty"
        },
        proc_macro2::Span::call_site(),
    );

    let path_method = if cfg!(feature = "dirs") {
        quote! {
            fn path() -> Result<std::path::PathBuf, derive_config::ConfigError> {
                let path = derive_config::dirs::config_dir().ok_or(derive_config::ConfigError::None)?;
                let name = env!("CARGO_PKG_NAME");
                let file = format!("{}.{}", name, #ext_name);

                Ok(path.join(file))
            }
        }
    } else {
        quote! {
            fn path() -> Result<std::path::PathBuf, derive_config::ConfigError> {
                let mut path = std::env::current_exe()?;
                path.set_file_name(env!("CARGO_CRATE_NAME"));
                path.set_extension(#ext_name);

                Ok(path)
            }
        }
    };

    quote! {
        impl #trait_ident for #struct_name {
            #path_method

            fn save(&self) -> Result<(), derive_config::ConfigError> {
                use std::io::Write;

                let path = Self::path()?;
                let mut file = std::fs::File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;

                let content = derive_config::#format_mod::#to_string_fn(&self)?;
                file.write_all(content.as_bytes())?;

                Ok(())
            }

            fn load() -> Result<Self, derive_config::ConfigError> {
                use std::io::{Read, Seek};

                let path = Self::path()?;
                let mut file = std::fs::File::open(&path)?;
                let mut text = String::new();
                file.read_to_string(&mut text)?;
                file.rewind()?;

                let config = derive_config::#format_mod::from_str(&text)?;

                Ok(config)
            }
        }
    }
}

fn derive_config(input: TokenStream, trait_name: &str, ext_name: &str) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = generate_impl(&struct_name, trait_name, ext_name);

    TokenStream::from(expanded)
}

#[cfg(feature = "json")]
#[proc_macro_derive(DeriveJsonConfig)]
pub fn derive_json_config(input: TokenStream) -> TokenStream {
    derive_config(input, "DeriveJsonConfig", "json")
}

#[cfg(feature = "toml")]
#[proc_macro_derive(DeriveTomlConfig)]
pub fn derive_toml_config(input: TokenStream) -> TokenStream {
    derive_config(input, "DeriveTomlConfig", "toml")
}

#[cfg(feature = "yaml")]
#[proc_macro_derive(DeriveYamlConfig)]
pub fn derive_yaml_config(input: TokenStream) -> TokenStream {
    derive_config(input, "DeriveYamlConfig", "yaml")
}
