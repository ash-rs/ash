use quote::format_ident;
use std::fmt::Display;
use syn::Ident;

/// Tries to prepend an underscore in case the name is not a valid identifier
pub fn snake_case_escape_ident(name: &str) -> Ident {
    syn::parse_str(name).unwrap_or_else(|_| format_ident!("_{name}"))
}

pub fn refpage_doc(target: &str, description: impl Display) -> String {
    let valid = matches!(
        target.get(0..2).map(|ab| ab.eq_ignore_ascii_case("vk")),
        Some(true)
    );

    let refpage = if valid {
        format!(
            "[Vulkan Manual Page]\
            (https://docs.vulkan.org/refpages/latest/refpages/source/{target}.html)"
        )
    } else {
        "<s>Vulkan Manual Page</s>".into()
    };

    format!("{} · {}", refpage, description)
}
