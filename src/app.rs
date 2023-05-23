use leptos::ev::MouseEvent;
use leptos::*;
use regex::Regex;
use std::collections::HashMap;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, String::new());
    let (word_count, set_word_count) = create_signal(cx, 0);
    let (character_count, set_character_count) = create_signal(cx, 0);
    let (dictionary, set_dictionary) = create_signal(cx, HashMap::new());

    let update_text = move |ev| {
        let v = event_target_value(&ev);
        set_text.set(v);
    };

    let show_result = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if text.get().is_empty() {
                return;
            }

            let word_occurrences = |text: String| {
                let mut occurrence: HashMap<String, u32> = HashMap::new();
                let re = Regex::new(r"\w+").unwrap();

                for word in re.find_iter(&text) {
                    let word = word.as_str().to_lowercase();
                    if occurrence.contains_key(&word) {
                        let _ = occurrence.entry(word.to_owned()).and_modify(|w| *w += 1);
                    } else {
                        let _ = occurrence.insert(word.to_owned(), 1);
                    }
                }
                occurrence
            };

            set_dictionary.set(word_occurrences(text.get()));
        });
    };

    let clear_input = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if text.get().is_empty() {
                return;
            }

            set_dictionary.set(HashMap::new());
            set_text.set(String::new());
        });
    };

    create_effect(cx, move |_| {
        set_word_count.set(text.get().split_whitespace().count());
        set_character_count.set(text.get().chars().count());
    });

    view! { cx,
        <main class="container md:mx-auto h-screen flex space-y-7">
            <div class="flex flex-col md:flex-row m-4 justify-around space-y-4 md:space-y-0 md:space-x-4">
                <textarea
                    id="text-input"
                    class="w-100 h-72 md:w-3/6"
                    placeholder="Enter text..."
                    prop:value={move || text.get()}
                    on:input=update_text
                />

                <div class="w-100 md:w-3/6 h-52 md:h-72 bg-white overflow-hidden lg:overflow-auto overflow-y-scroll">
                    <div class="snap-y text-left">
                        {
                            move || dictionary.get().iter().map(|(key, value)| {
                                view! {cx,
                                <p>{format!("{} - {}", key, value)}</p>
                                }
                            }).collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </div>

            <div class="box-border h-24 w-auto p-4 bg-blue-200 m-4">
                <p>{move || format!("Words: {}", word_count.get())}</p>
            </div>

            <div class="box-border h-24 w-auto p-4 bg-blue-200 m-4">
                <p>{move || format!("Characters: {}", character_count.get())}</p>
            </div>

            <div class="flex flex-row space-x-4 justify-center">
                <button class="rounded-full p-2.5 bg-blue-400 text-white" on:click=show_result>
                    {"Show Result"}
                </button>
                <button class="rounded-full p-2.5 bg-blue-400 text-white" on:click=clear_input>
                    {"Clear Input"}
                </button>
            </div>
        </main>
    }
}
