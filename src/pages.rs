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
            a href="/" { "GoodCoffee.xyz" }
          }
          nav {}
          div {}
        }
        main {
          div ."index-atf" {
            h1 { "GoodCoffee.xyz" }
            p { "Check out some coffee reviews!" }
          }
        }
        footer {}
      }
    }
  }
}
