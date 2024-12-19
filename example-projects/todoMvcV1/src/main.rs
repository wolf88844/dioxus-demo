#[allow(non_snake_case)]
use dioxus::prelude::*;
use std::collections::HashMap;

const STYLE:Asset = asset!("/assets/todomvc.css");
const ICON:Asset = asset!("/assets/favicon.ico");

fn main(){    
    dioxus::launch(app);
}

#[component]
fn app1()->Element{
    rsx!{
        img{src:ICON}
    }
}

#[derive(PartialEq,Eq,Clone,Copy)]
enum FilterState{
    All,
    Active,
    Completed
}

struct TodoItem{
    checked:bool,
    content:String,
}
#[component]
fn app()->Element{
    let mut todos = use_signal(HashMap::<u32,TodoItem>::new);
    let filter = use_signal(|| FilterState::All);

    let active_todo_count = use_memo(move|| todos.read().values().filter(|item|!item.checked).count());
    let filterd_todos = use_memo(move||{
        let mut filterd_todos = todos
        .read()
        .iter()
        .filter(|(_,item)|match filter(){
            FilterState::All=>true,
            FilterState::Active=>!item.checked,
            FilterState::Completed=>item.checked
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
        filterd_todos.sort_unstable();

        filterd_todos
    });

    let toggle_all = move |_|{
        let check = active_todo_count()!=0;
        for(_,item) in todos.write().iter_mut(){
            item.checked=check;
        }
    };

    rsx!{
        document:: Link{rel:"stylesheet",href:STYLE}
        section { 
            class:"todoapp",
            TodoHeader {todos}
            section { 
                class:"main",
                if !todos.read().is_empty(){
                input {
                        id:"toggle-all",
                        class:"toggle-all",
                        r#type:"checkbox",
                        onchange:toggle_all,
                        checked:active_todo_count()==0,
                }
                    label{r#for:"toggle-all"}
                     }

                     ul { 
                        class:"todo-list",
                        for id in filterd_todos(){
                            TodoEntry{key:"{id}",id,todos}
                        }
                      }

                      if !todos.read().is_empty(){
                        ListFooter{active_todo_count,todos,filter}
                      }

                }
             }

             footer { 
                class:"info",
                p{"Double-click to edit a todo"}
                p{
                    "Created by "
                    a{href:""}
        }
                p { 
                    "Part of "
                    a{href:"","TodoMVC"}
                 }
    }
}

    }
#[component]
fn TodoHeader(mut todos:Signal<HashMap<u32,TodoItem>>)->Element{
    let mut draft = use_signal(||"".to_string());
    let mut todo_id = use_signal(||0);

    let onekeydown = move |evt:KeyboardEvent|{
        if evt.key()==Key::Enter && !draft.read().is_empty(){
            let id = todo_id();
            let todo = TodoItem{
                checked:false,
                content: draft.to_string(),
            };
            todos.write().insert(id,todo);
            todo_id+=1;
            draft.set("".to_string());
        }
    };
    rsx!{
        header { 
            class:"header",
            h1{"todos"},
            input { 
                class:"new-todo",
                placeholder:"What needs to be done?",
                value:"{draft}",
                autofocus:true,
                oninput:move|evt| draft.set(evt.value()),
                onkeydown:onekeydown,
             }
        }
         }

    }


#[component]
fn TodoEntry(mut todos:Signal<HashMap<u32,TodoItem>>,id:u32)->Element{
    let mut is_editing = use_signal(||false);
    let checked = use_memo(move||todos.read().get(&id).unwrap().checked);
    let contents = use_memo(move||todos.read().get(&id).unwrap().content.clone());

    rsx!{
        li { 
            class: if checked(){"completed"},
            class: if is_editing(){"editing"},
            div { 
                class:"view",
                input { 
                    class:"toggle",
                    r#type:"checkbox",
                    id:"cbg-{id}",
                    checked:"{checked}",
                    oninput:move|evt| todos.write().get_mut(&id).unwrap().checked=evt.checked(),
                 }
                 label { 
                    r#for:"cbg-{id}",
                    ondoubleclick:move|_| is_editing.set(true),
                    onclick:|evt|evt.prevent_default(),
                    "{contents}"
                  }
                  button { 
                    class:"destory",
                    onclick:move|evt|{
                        evt.prevent_default();
                        todos.write().remove(&id);
                    },
                   }

             }

             if is_editing(){
                input { 
                    class:"edit",
                    value:"{contents}",
                    oninput:move|evt| todos.write().get_mut(&id).unwrap().content=evt.value(),
                    onblur:move|_| is_editing.set(false),
                    onkeydown:move|evt|{
                        match evt.key(){
                            Key::Enter|Key::Escape|Key::Tab=>is_editing.set(false),
                            _=>{}
                        }
                    },
                 }
              }
             }
         }
    }
    
#[component]
fn ListFooter(
    mut todos:Signal<HashMap<u32,TodoItem>>,
    active_todo_count:ReadOnlySignal<usize>,
    mut filter:Signal<FilterState>,
)->Element{
    let show_clear_completed = use_memo(move||todos.read().values().any(|todo|todo.checked));
    rsx!{
        footer { 
            class:"footer",
            span { 
                class:"todo-count",
                strong {"{active_todo_count}"}
                span { 
                    match active_todo_count(){
                        1=> "item",
                        _=>"items",
                    }
                    " left"
                 }
             }
             ul { 
                class:"filters",
                for(state,state_text,url) in [
                    (FilterState::All,"All","#/"),
                    (FilterState::Active,"Active","#/active"),
                    (FilterState::Completed,"Completed","#/completed"),
                ]{
                    li { 
                        a { 
                            href:url,
                            class: if filter()==state{"selected"},
                            onclick:move|evt|{
                                evt.prevent_default();
                                filter.set(state);
                            },
                            {state_text}
                         }
                     }
                }

              }
              if show_clear_completed(){
                button { 
                    class:"clear-completed",
                    onclick:move|_| todos.write().retain(|_,todo|!todo.checked),
                    "Clear completed"
                 }
              }
         }

    }
}