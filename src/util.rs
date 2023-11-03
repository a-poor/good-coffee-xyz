use maud::{html, Markup, PreEscaped};


pub const GLOBAL_CSS: PreEscaped<&'static str> = PreEscaped("<link rel=\"stylesheet\" href=\"/static/global.css\" />");

pub const HTMX: PreEscaped<&'static str> = PreEscaped("<script src=\"/static/htmx.min.js\"></script>");

pub const ALPINE_JS: PreEscaped<&'static str> = PreEscaped("<script src=\"/static/alpinejs.min.js\" defer></script>");


pub fn load_assets() -> Markup {
    html! {
        (GLOBAL_CSS)
        (HTMX)
        (ALPINE_JS)
    }
}
