use crate::util::load_assets;
use maud::{html, Markup, DOCTYPE};


pub async fn index() -> Markup {
  html! {
    (DOCTYPE)
    html {
      head {
        title { "GoodCoffee.xyz" }
        (load_assets())
      }
      body {
        header {
          div {
            a.logo href="/" { "GoodCoffee.xyz" }
          }
          div style="flex-grow: 1;" {}
          nav {
            ul {
              li { a href="/" { "Home" } }
              li { a href="/reviews" { "Reviews" } }
              li { a href="/about" { "About" } }
            }
          }
        }
        main {
          div ."index-atf" {
            h1 { "Good Coffee" }
            p { "Check out some of my coffee reviews!" }
          }
        }
        footer {}
      }
    }
  }
}
