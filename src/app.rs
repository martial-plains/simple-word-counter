use leptos::{ev::MouseEvent, *};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::OnceLock};

use crate::{
    components::{StatisticsOptionsPanel, ToggleSwitch},
    utils::word_count,
};

static WORD_REGEX: OnceLock<Regex> = OnceLock::new();

#[repr(usize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatisticOption {
    Characters,
    Paragraphs,
    ReadingTime,
    Sentences,
    SpeakingTime,
    Words,
}

#[component]
pub fn options_dialog<F>(
    cx: Scope,
    onbutton_done: F,
    statistics_options: RwSignal<Vec<StatisticOption>>,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let show_words = create_rw_signal(
        cx,
        statistics_options.get().contains(&StatisticOption::Words),
    );

    let show_characters = create_rw_signal(
        cx,
        statistics_options
            .get()
            .contains(&StatisticOption::Characters),
    );

    let show_sentences = create_rw_signal(
        cx,
        statistics_options
            .get()
            .contains(&StatisticOption::Sentences),
    );

    let show_paragraphs = create_rw_signal(
        cx,
        statistics_options
            .get()
            .contains(&StatisticOption::Paragraphs),
    );

    let show_reading_time = create_rw_signal(
        cx,
        statistics_options
            .get()
            .contains(&StatisticOption::ReadingTime),
    );

    let show_speaking_time = create_rw_signal(
        cx,
        statistics_options
            .get()
            .contains(&StatisticOption::SpeakingTime),
    );

    create_effect(cx, move |_| {
        let mut options = Vec::new();

        if show_words.get() {
            options.push(StatisticOption::Words);
        }

        if show_characters.get() {
            options.push(StatisticOption::Characters);
        }

        if show_sentences.get() {
            options.push(StatisticOption::Sentences);
        }

        if show_paragraphs.get() {
            options.push(StatisticOption::Paragraphs);
        }

        if show_reading_time.get() {
            options.push(StatisticOption::ReadingTime);
        }

        if show_speaking_time.get() {
            options.push(StatisticOption::SpeakingTime);
        }

        statistics_options.set(options);
    });

    view! { cx,
        <div id="dialog"
            class="hidden fixed z-50 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 bg-white shadow-md rounded-md px-8 py-6 space-y-5 drop-shadow-lg dark:bg-slate-800">
            <h1 class="text-2xl font-semibold">{"Options"}</h1>

            <form class="pb-8 mb-4 h-[400px] overflow-auto">
                <div class="mb-4">
                    <ToggleSwitch label="Words" value=show_words/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Characters" value=show_characters/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Sentences" value=show_sentences/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Paragraphs" value=show_paragraphs/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Reading Time" value=show_reading_time/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Speaking Time" value=show_speaking_time/>
                </div>
            </form>

            <div class="flex justify-end">
                <button id="done" class="px-5 py-2 bg-indigo-500 hover:bg-indigo-700 text-white cursor-pointer rounded-md" on:click=onbutton_done>
                    {"Done"}</button>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Copy)]
struct GlobalState {
    text: RwSignal<String>,
    word_total: RwSignal<usize>,
    character_total: RwSignal<usize>,
    dictionary: RwSignal<HashMap<String, u32>>,
    statistics_options: RwSignal<Vec<StatisticOption>>,
}

impl GlobalState {
    fn new(cx: Scope) -> Self {
        let storage = window().local_storage().unwrap().unwrap();

        let text = create_rw_signal(cx, storage.get_item("text").unwrap().unwrap_or_default());
        let word_total = create_rw_signal(cx, 0);
        let character_total = create_rw_signal(cx, 0);
        let dictionary = create_rw_signal(cx, HashMap::new());
        let statistics_options = create_rw_signal(
            cx,
            storage
                .get_item("statistics_options")
                .unwrap()
                .map(|s| serde_json::from_str(&s).unwrap())
                .unwrap_or(vec![
                    StatisticOption::Words,
                    StatisticOption::Characters,
                    StatisticOption::Sentences,
                    StatisticOption::Paragraphs,
                    StatisticOption::ReadingTime,
                    StatisticOption::SpeakingTime,
                ]),
        );

        Self {
            text,
            word_total,
            character_total,
            dictionary,
            statistics_options,
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let storage = window().local_storage().unwrap().unwrap();
    let state = GlobalState::new(cx);
    provide_context(cx, state);

    let update_text = move |ev| {
        let value: String = event_target_value(&ev);
        storage.set_item("text", &value).unwrap();
        state.text.set(value);
    };

    let get_result = move || {
        spawn_local(async move {
            if state.text.get().is_empty() {
                return;
            }

            let word_occurrences = |text: String| {
                let mut occurrence: HashMap<String, u32> = HashMap::new();
                let re = WORD_REGEX.get_or_init(|| Regex::new(r"\w+").unwrap());
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

            state.dictionary.set(word_occurrences(state.text.get()));
        });
    };

    let clear_input = move |ev: MouseEvent| {
        ev.prevent_default();
        let storage = window().local_storage().unwrap().unwrap();
        spawn_local(async move {
            if state.text.get().is_empty() {
                return;
            }

            state.dictionary.set(HashMap::new());
            state.text.set(String::new());
            storage.set_item("text", "").unwrap();
        });
    };

    let open_dialog = |_: MouseEvent| {
        let dialog = document().get_element_by_id("dialog").unwrap();
        let overlay = document().get_element_by_id("overlay").unwrap();

        dialog.class_list().remove_1("hidden").unwrap();
        overlay.class_list().remove_1("hidden").unwrap();
    };

    let close_dialog = |_: MouseEvent| {
        let dialog = document().get_element_by_id("dialog").unwrap();
        let overlay = document().get_element_by_id("overlay").unwrap();

        dialog.class_list().add_1("hidden").unwrap();
        overlay.class_list().add_1("hidden").unwrap();
    };

    create_effect(cx, move |_| {
        let storage = window().local_storage().unwrap().unwrap();
        state.word_total.set(word_count(state.text.get().as_str()));
        state.character_total.set(state.text.get().chars().count());
        get_result();

        storage
            .set_item(
                "statistics_options",
                &json!(state.statistics_options.get()).to_string(),
            )
            .unwrap();
    });

    view! { cx,
        <main class="md:mx-auto container h-screen">

            <div id="overlay" class="fixed hidden z-40 w-screen h-screen inset-0 bg-gray-900 bg-opacity-60"></div>

            <OptionsDialog onbutton_done=close_dialog statistics_options=state.statistics_options />

            <div class="space-y-7">
                <div class="flex">
                    <button class="px-5 py-2 bg-rose-500 hover:bg-rose-700 text-white cursor-pointer rounded-md flex mr-auto" on:click=clear_input>
                        { "Clear" }
                    </button>
                    <button id="open" class="px-5 py-2 bg-rose-500 hover:bg-rose-700 text-white cursor-pointer rounded-md flex ml-auto" on:click=open_dialog>
                        { "Show Options" }
                    </button>
                </div>

                <div class="lg:flex bg-gray-200 p-2 mb-6 mt-auto mb-auto dark:bg-gray-800">
                    <div class="lg:w-8/12 p-2">
                        <textarea
                        class="block w-full h-96 lg:h-full p-2 mb-1 border-2 border-gray-400 rounded-lg focus:outline-none dark:bg-black"
                        placeholder="Enter text here"
                        prop:value={move || state.text.get()}
                        on:input=update_text></textarea>
                    </div>
                    <div class="lg:w-4/12 p-2">
                        {
                            move || view! {cx, <StatisticsOptionsPanel statistics_options=state.statistics_options.get() word_total=state.word_total.get() character_total=state.character_total.get() text=state.text.get()/>}
                        }
                        <div class="bg-white p-3 rounded-md border-2 border-gray-700 dark:bg-gray-800">
                            <div class="text-3xl mt-2 mb-4 h5">{"Keyword Density"}</div>
                            <div class="relative overflow-auto h-full max-h-56 mb-4 border-b-2">
                                {move || if state.text.get().is_empty() {
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
                                                    let dictionary = state.dictionary.get();
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
            </div>
        </main>
    }
}
