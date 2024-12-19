
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // Initialize the logger with debug level
    dioxus_logger::init(Level::DEBUG).expect("failed to initialize logger");
    
    dioxus::launch(app);
}

#[component]
fn App() -> Element {
    rsx! {
        head {
            link { rel: "icon", r#type: "image/x-icon", href: FAVICON }
        }
        Router::<Route> {
            config:||{
                RouterConfig::default()
                .on_update(|state|{
                    (state.current() == Route::BlogList{})
                    .then_some(NavigationTarget::Internal(Route::Home {  }))
                })
            }
        }
    }
}

#[derive(Routable,Clone,PartialEq)]
#[rustfmt::skip]
enum Route{
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/favicon.ico")]
    Icon {},
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
    #[route("/:..route")]
    PageNotFound { route:Vec<String> }

}

#[component]
pub fn app()->Element{
    rsx!{
        Router<Route> {}
    }
}

#[component]
fn Icon()->Element{
    log::info!("Icon");
    rsx!{
        img { src: FAVICON }
    }
}

#[component]
fn Home()->Element{
    log::info!("Home");
    let nav = navigator();
    nav.push(Route::PageNotFound { route: vec![] });
    nav.replace(Route::Home {  });
    nav.go_back();
    nav.go_forward();
    rsx!{
        GoBackButton { "返回"}
        GoForwardButton{ "前进"}
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

#[component]
fn PageNotFound(route:Vec<String>) ->Element{
    rsx!{
        h1 {"Page not found"}
        p {"We are terribly sorry, but the page you requested could not be found."}
        pre {color:"red","log:\nattempted to open {route:?}"}
    }
}

#[component]
fn NavBar()->Element{
    rsx!{
        nav { 
            ul {
                li {
                    Link {to:Route::Home{},"Home"}
                }
                li { 
                    Link {to:Route::BlogList{},"Blog"}
                    }
                li { 
                    Link{to:Route::BlogList{},"My Blog"}
                 }

            }
         }
         Outlet::<Route>{}
                        }
                    }

#[component]
fn Blog()->Element{
    rsx!(
        h1 { "Blog" }
        Outlet::<Route>{}
    )
}

#[component]
fn BlogPost(name:String)->Element{
    rsx!{
        h2 { "Blog Post: {name}" }
    }
}

#[component]
fn BlogList()->Element{
    rsx!{
        h2{"Choose a post"}
        ul { 
            li{
                Link {
                    to: Route::BlogPost{
                        name:"Blog post 1".into(),
                    },
                    "Read the first blog post"
            }
            }
            li{
                Link {
                    to: Route::BlogPost{
                        name:"Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}