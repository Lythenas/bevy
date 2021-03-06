use find_crate::Manifest;
use proc_macro::TokenStream;
use syn::Path;

#[derive(Debug)]
pub struct Modules {
    pub bevy_reflect: String,
}

impl Modules {
    pub fn meta(name: &str) -> Modules {
        Modules {
            bevy_reflect: format!("{}::reflect", name),
        }
    }

    pub fn external() -> Modules {
        Modules {
            bevy_reflect: "bevy_reflect".to_string(),
        }
    }

    pub fn internal() -> Modules {
        Modules {
            bevy_reflect: "crate".to_string(),
        }
    }
}

pub fn get_modules() -> Modules {
    let manifest = Manifest::new().unwrap();
    if let Some(package) = manifest.find(|name| name == "bevy") {
        Modules::meta(&package.name)
    } else if let Some(package) = manifest.find(|name| name == "bevy_internal") {
        Modules::meta(&package.name)
    } else if let Some(_package) = manifest.find(|name| name == "bevy_reflect") {
        Modules::external()
    } else {
        Modules::internal()
    }
}

pub fn get_path(path_str: &str) -> Path {
    syn::parse(path_str.parse::<TokenStream>().unwrap()).unwrap()
}
