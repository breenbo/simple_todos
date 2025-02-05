use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css")
        },
        Title{},
        ManageTodos{}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            class: "pl-4 py-8 text-pink-500  text-3xl font-semibold bg-pink-100",
            "Simple Todo List"
        }
    }
}

#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    name: String,
    done: bool,
}
#[derive(Clone, PartialEq)]
struct TodoList {
    list: Vec<Todo>,
}
#[derive(Clone, PartialEq, Props)]
struct TodoDone {
    list: Vec<Todo>,
}

// TODO: add TodoDone as prop to displayTodo, and move todos from TodoList to TodoDone

#[component]
fn ManageTodos() -> Element {
    // share the state because need for add/delete
    // signal to have reactive state
    use_context_provider(|| Signal::new(TodoList { list: vec![] }));

    rsx!( div {
        class:"pl-4",
    AddTodo {}, DisplayTodos {},
    })
}

fn add_todo(mut todo_list: Signal<TodoList>, mut new_name: Signal<String>) {
    // consume the input value because not needed anymore, or use clone()
    let len = todo_list.read().list.len();
    let id = if len == 0 {
        1
    } else {
        todo_list.read().list[len - 1].id + 1
    };

    let new_todo = Todo {
        id,
        name: new_name.take(),
        done: false,
    };
    let old_list = todo_list.read().list.clone();

    let list = [old_list, vec![new_todo]].concat();
    todo_list.set(TodoList { list });
}

fn on_enter(evt: KeyboardEvent, input: Signal<String>, todo_list: Signal<TodoList>) {
    if evt.key() == Key::Enter && !input.read().trim().is_empty() {
        add_todo(todo_list, input);
    }
}

#[component]
fn AddTodo() -> Element {
    let mut input = use_signal(|| "".to_string());
    let todo_list = use_context::<Signal<TodoList>>();

    rsx! {
        div {
            class: "w-full py-8 flex items-center  gap-x-6",
            input {
                class:"w-[300px] py-2 px-3 border border-pink-500 rounded-lg",
                placeholder: "Add new todo",
                oninput: move |event| input.set(event.value()),
                onkeydown: move |event| on_enter(event, input, todo_list),
                value: "{input}"
            },
            button {
                class: "bg-pink-800 py-2 px-6 text-white font-semibold rounded-full hover:bg-pink-600 hover:shadow-md transition",
                onclick: move |_| {
                    if input.read().trim().is_empty() {
                        return;
                    }

                    add_todo(todo_list, input);
                },
                "Add"
            }
        }
    }
}

#[component]
fn DisplayTodos() -> Element {
    let todo_list = use_context::<Signal<TodoList>>();

    rsx! {
        div {
            for todo in &todo_list.read().list {
                DisplayTodo { id: todo.id, name: todo.name.clone(), done: todo.done }
            }
        }
    }
}

fn todo_done(mut todo_list: Signal<TodoList>, id: usize) {
    let current_list = todo_list.read().list.clone();
    let new_list: Vec<Todo> = current_list
        .into_iter()
        .map(|t| {
            if t.id == id {
                Todo {
                    id: t.id,
                    name: t.name,
                    done: !t.done,
                }
            } else {
                Todo {
                    id: t.id,
                    name: t.name,
                    done: t.done,
                }
            }
        })
        .collect();

    todo_list.set(TodoList { list: new_list });
}

fn delete_todo(mut todo_list: Signal<TodoList>, id: usize) {
    let current_list = todo_list.read().list.clone();
    let new_list: Vec<Todo> = current_list.into_iter().filter(|t| t.id != id).collect();
    todo_list.set(TodoList { list: new_list });
}

#[component]
fn DisplayTodo(id: usize, name: String, done: bool) -> Element {
    let todo_list = use_context::<Signal<TodoList>>();

    rsx!(
        div {
            class:"w-full flex",
            div {
                class:"p-2 mb-4 w-10/12 flex items-center justify-between border border-pink-100 border-t-0 border-x-0",
                span {
                    class: if done {"line-through"} else {""},
                    "{id} - {name}",
                },
                div {
                    class:"flex gap-x-4",
                    if done {
                        button {
                            class:"bg-red-300 px-2 py-1 rounded-lg hover:bg-red-400 transition",
                            onclick: move |_| { delete_todo(todo_list, id); },
                            "Delete"
                        }
                    }
                    button {
                        class:
                            if done {
                                "bg-cyan-400 px-2 py-1 rounded-lg hover:bg-cyan-500 transition"
                            } else {
                                "bg-lime-400 px-2 py-1 rounded-lg hover:bg-lime-500 transition"
                            },
                        onclick: move |_| { todo_done(todo_list, id); },
                        if done {"Undo"} else { "Done" }
                    }
                }
            }
        }
    )
}
