pub fn render(template_str: &str, args: &[(&str, &str)]) -> String {
    let mut render = template_str.to_string();
    for (variable, value) in args {
        let pattern = format!("{{{{{}}}}}", variable);
        render = render.replace(&pattern, value);
    }
    render
}
