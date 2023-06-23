use leptos::ev::MouseEvent;
use leptos::*;
use regex::Regex;
use std::collections::HashMap;

use std::time::Duration;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let storage = window().local_storage().unwrap().unwrap();

    let (text, set_text) = create_signal(
        cx,
        storage.get_item("text").unwrap().unwrap_or(String::new()),
    );
    let (word_count, set_word_count) = create_signal(cx, 0);
    let (character_count, set_character_count) = create_signal(cx, 0);
    let (dictionary, set_dictionary) = create_signal(cx, HashMap::new());

    let update_text = move |ev| {
        let value: String = event_target_value(&ev);
        storage.set_item("text", &value).unwrap();
        set_text.set(value);
    };

    let get_result = move || {
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
        get_result()
    });

    view! { cx,
        <main class="container md:mx-auto h-screen flex space-y-7">
            <div class="lg:flex bg-gray-200 p-2 border-2 border-gray-400 rounded-lg mb-6 mt-auto mb-auto dark:bg-gray-800">
                <div class="lg:w-8/12 p-2">
                    <textarea
                    class="block w-full h-96 lg:h-full p-2 mb-1 focus:outline-none dark:bg-black"
                    placeholder="Enter text here"
                    prop:value={move || text.get()}
                    on:input=update_text></textarea>
                </div>
                <div class="lg:w-4/12 p-2">
                    <div class="mb-4 bg-white p-3 rounded-md border-2 border-gray-700 text-gray-500 dark:bg-gray-800">
                        <div class="h2 text-3xl text-black mt-2 mb-4 dark:text-white">{"Statistics"}</div>
                        <div class="border-b-2 border-gray-700 flex justify-between mb-4">
                            <div class="w-2/5">
                                <div class="uppercase text-xs">{"Words"}</div>
                                <span class="text-4xl font-bold text-black dark:text-white">{word_count}</span>
                            </div>
                            <div class="w-2/5">
                                <div class="uppercase text-xs">{"Characters"}</div>
                                <span class="text-4xl font-bold text-black dark:text-white">{character_count}</span>
                            </div>
                        </div>
                        <div class="border-b-2 border-gray-700 flex justify-between mb-4">
                            <div class="w-2/5">
                                <div class="uppercase text-xs">{"Sentences"}</div>
                                <span class="text-4xl font-bold text-black dark:text-white">{move || sentence_count(text.get())}</span>
                            </div>
                            <div class="w-2/5">
                                <div class="uppercase text-xs">{"Paragraphs"}</div>
                                <span class="text-4xl font-bold text-black dark:text-white">{move || paragraph_count(text.get())}</span>
                            </div>
                        </div>
                        <div class="border-b-2 border-gray-700 flex justify-between mb-4">
                            <div class="w-2/5">
                                <div class="uppercase text-xs whitespace-nowrap">
                                    {"Reading Time"}
                                    <span title="Based on 275 words per minute" class="inline-block">
                                        <i class="fa-solid fa-circle-question"></i>
                                    </span>
                                </div>
                                <div class="flex flex-nowrap">
                                    <span class="text-3xl text-black dark:text-white">{move || format_duration(cx, calculate_duration(word_count.get(), 275))}</span>
                                </div>
                            </div>
                            <div class="w-2/5">
                                <div class="uppercase text-xs whitespace-nowrap">
                                    {"Speaking Time"}
                                    <span title="Based on 180 words per minute" class="inline-block">
                                        <i class="fa-solid fa-circle-question"></i>
                                    </span>
                                </div>
                                <div class="flex flex-nowrap">
                                    <span class="text-3xl text-black dark:text-white">{move || format_duration(cx, calculate_duration(word_count.get(), 180))}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="bg-white p-3 rounded-md border-2 border-gray-700 dark:bg-gray-800">
                        <div class="text-3xl mt-2 mb-4 h5">{"Keyword Density"}</div>
                        <div class="relative overflow-auto h-full max-h-56 mb-4 border-b-2">
                            {move || if text.get().is_empty() {
                                view! {cx,
                                    <>
                                        <p>{"Start typing to get a list of keywords that are most used"}</p>
                                    </>
                                }
                            } else {
                                view! {cx,
                                <>
                                    <ul>
                                        {
                                            move || {
                                                let dictionary = dictionary.get();
                                                let mut dictionary = dictionary.iter().collect::<Vec<_>>();
                                                dictionary.sort_by(|a, b| a.1.cmp(b.1));
                                                dictionary.iter().enumerate().rev().map(|(index, (key, value))| {
                                                    view! {cx,
                                                        <li class=format!("keywords-item flex justify-between items-center px-2 {} dark:bg-gray-800", if index % 2 == 0 { "bg-gray-300" } else { "bg-white" })>
                                                            <div class="inline-block overflow-hidden overflow-ellipsis text-sm">{key.to_string()}</div>
                                                            <div class="flex items-baseline text-gray-700 ">
                                                                <span class="font-semibold dark:text-white">{value.to_string()}</span>
                                                                <span class="text-xs w-14 text-right dark:text-white">{format!("{:.2}%", (**value as f32 / dictionary.len() as  f32) * 100.0)}</span>
                                                            </div>
                                                        </li>
                                                    }
                                                }).collect::<Vec<_>>()
                                            }
                                        }
                                    </ul>
                                </>
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}

fn sentence_count(text: String) -> usize {
    let pattern = Regex::new(r"(?i)[^.!?]+[.!?]").unwrap();
    let matches = pattern.find_iter(&text);
    let sentences: Vec<&str> = matches.map(|m| m.as_str()).collect();

    sentences.len()
}

fn paragraph_count(text: String) -> usize {
    if text.is_empty() {
        return 0;
    }

    let pattern = Regex::new(r"\n\s*\n").unwrap();
    let paragraphs: Vec<&str> = pattern.split(&text).collect();

    paragraphs.len()
}

fn calculate_duration(word_count: usize, words_per_minute: u32) -> Duration {
    let minutes = f64::from(word_count as u32) / f64::from(words_per_minute);
    let seconds = minutes * 60.0;

    Duration::from_secs(seconds as u64)
}

fn format_duration(cx: Scope, duration: Duration) -> impl IntoView {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes > 60 {
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;

        view! {cx,
            <>
                <div class="flex flex-nowrap text-center space-x-2">
                    <span class="text-3xl text-black dark:text-white">
                        {hours}
                        <div class="text-xs">{ if hours != 1 { "hrs" } else { "hr" } }</div>
                    </span>
                    <span class="text-3xl text-black dark:text-white">
                        {remaining_minutes}
                        <div class="text-xs">{ if remaining_minutes != 1 { "mins" } else { "min" } }</div>
                    </span>
                    <span class="text-3xl text-black dark:text-white">
                        {seconds}
                        <div class="text-xs">{  if seconds != 1 { "secs" } else { "sec" } }</div>
                    </span>
                </div>
            </>
        }
    } else if minutes > 0 {
        view! {cx,
            <>
                <div class="flex flex-nowrap text-center space-x-2">
                    <span class="text-3xl text-black dark:text-white">
                        {minutes}
                        <div class="text-xs">{ if minutes != 1 { "mins" } else { "min" } }</div>
                    </span>
                    <span class="text-3xl text-black dark:text-white">
                        {seconds}
                        <div class="text-xs">{ if seconds != 1 { "secs" } else { "sec" } }</div>
                    </span>
                </div>
            </>
        }
    } else {
        view! {cx,
            <>
                <div class="flex flex-nowrap text-center">
                    <span class="text-3xl text-black dark:text-white">
                        {seconds}
                        <div class="text-xs">{"secs"}</div>
                    </span>
                </div>
            </>
        }
    }
}
