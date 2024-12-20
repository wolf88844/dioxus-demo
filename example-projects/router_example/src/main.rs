
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    // Initialize the logger with debug level
    dioxus_logger::init(Level::DEBUG).expect("failed to initialize logger");
    
    dioxus::launch(app);
}

#[component]
fn App() -> Element {
    rsx! {
        head {
            link {
                rel: "icon",
                r#type: "image/x-icon",
                href: asset!("/assets/favicon.ico"),
            }
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/router.css") }
        Router::<Route> {}
    }
    }

#[derive(Routable,Clone,PartialEq,Debug)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route{
    #[layout(NavBar)]
    #[route("/")]
    Home {},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},
                #[route("/:name")]
                BlogPost { name:String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
        #[redirect("/",|| Route::BlogList{})]
        #[redirect("/:name",|name:String| Route::BlogPost{name})]
    #[end_nest]
    #[route("/dogge")]
    Dogge {},
    #[route("/:..route")]
    PageNotFound { route:Vec<String> }

}

#[component]
pub fn app()->Element{
    rsx!{
        Router::<Route> {}
    }
}


#[component]
fn Home()->Element{
    log::info!("Home");
    rsx!{
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

#[component]
fn PageNotFound(route:Vec<String>) ->Element{
    rsx!{
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested could not be found." }
        pre { color: "red", "log:\nattempted to open {route:?}" }
    }
}

#[component]
fn NavBar()->Element{
    rsx!{
        nav { id: "navbar",
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::BlogList {}, "Blog" }
                }
                li {
                    Link { to: Route::BlogList {}, "My Blog" }
                }
                li {
                    Link { to: Route::Dogge {}, "Dogge" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Blog()->Element{
    rsx!(
        h1 { "Blog" }
        Outlet::<Route> {}
    )
}

#[component]
fn BlogPost(name:String)->Element{
    let contents = match name.as_str(){
        "Blog post 1"=> "This is the first blog post",
        "Blog post 2"=> "This is the second blog post",
        _=> "This blog post does not exist"
    };
    rsx!{
        h2 { "Blog Post: {name}" }
        p { "{contents}" }
    }
}

#[component]
fn BlogList()->Element{
    rsx!{
        h2 { "Choose a post" }
        div { id: "blog-list",
            Link {
                to: Route::BlogPost {
                    name: "Blog post 1".into(),
                },
                "Read the first blog post"
            }
            Link {
                to: Route::BlogPost {
                    name: "Blog post 2".into(),
                },
                "Read the second blog post"
            }
        }
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[component]
fn Dogge() -> Element {
    let mut future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    match &*future.read_unchecked(){
        Some(Ok(response))=>rsx!{
            button { onclick: move |_| future.restart(), "Click to fetch another dog" }
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{response.message}",
                }
            }
        },
        Some(Err(_))=>rsx!{
            div { "Failed to fetch dog" }
        },
        None=>rsx!{
            div { "Loading..." }
        }
    }
    
}