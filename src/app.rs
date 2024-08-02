use leptos::{ev::MouseEvent, *};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::LazyLock};

use crate::components::{StatisticsOptionsPanel, ToggleSwitch};

static WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\w+").unwrap());
static SENTENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)[^.!?]+[.!?]").unwrap());
static PARAGRAPH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n\s*\n").unwrap());

#[repr(usize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatisticOption {
    Characters,
    CharacterCountNoSpaces,
    LineCount,
    Paragraphs,
    ReadingTime,
    Sentences,
    LongestSentenceWords,
    ShortestSentenceWords,
    AvgSentenceWords,
    AvgWordLength,
    SpeakingTime,
    UniqueWords,
    Words,
}

#[component]
pub fn options_dialog<F>(
    onbutton_done: F,
    statistics_options: RwSignal<Vec<StatisticOption>>,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let show_words = create_rw_signal(statistics_options.get().contains(&StatisticOption::Words));

    let show_unique_words = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::UniqueWords),
    );

    let show_characters = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::Characters),
    );

    let show_character_count_no_spaces = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::CharacterCountNoSpaces),
    );

    let show_sentences = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::Sentences),
    );

    let show_longest_sentence_words = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::LongestSentenceWords),
    );

    let show_shortest_sentence_words = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::ShortestSentenceWords),
    );

    let show_avg_sentence_words = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::AvgSentenceWords),
    );

    let show_avg_word_length = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::AvgWordLength),
    );

    let show_paragraphs = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::Paragraphs),
    );

    let show_line_count = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::LineCount),
    );

    let show_reading_time = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::ReadingTime),
    );

    let show_speaking_time = create_rw_signal(
        statistics_options
            .get()
            .contains(&StatisticOption::SpeakingTime),
    );

    create_effect(move |_| {
        let mut options = Vec::new();

        if show_words.get() {
            options.push(StatisticOption::Words);
        }

        if show_unique_words.get() {
            options.push(StatisticOption::UniqueWords);
        }

        if show_characters.get() {
            options.push(StatisticOption::Characters);
        }

        if show_character_count_no_spaces.get() {
            options.push(StatisticOption::CharacterCountNoSpaces);
        }

        if show_sentences.get() {
            options.push(StatisticOption::Sentences);
        }

        if show_longest_sentence_words.get() {
            options.push(StatisticOption::LongestSentenceWords);
        }

        if show_shortest_sentence_words.get() {
            options.push(StatisticOption::ShortestSentenceWords);
        }

        if show_avg_sentence_words.get() {
            options.push(StatisticOption::AvgSentenceWords);
        }

        if show_avg_word_length.get() {
            options.push(StatisticOption::AvgWordLength);
        }

        if show_paragraphs.get() {
            options.push(StatisticOption::Paragraphs);
        }

        if show_line_count.get() {
            options.push(StatisticOption::LineCount);
        }

        if show_reading_time.get() {
            options.push(StatisticOption::ReadingTime);
        }

        if show_speaking_time.get() {
            options.push(StatisticOption::SpeakingTime);
        }

        statistics_options.set(options);
    });

    view! {
        <div id="dialog"
            class="hidden fixed z-50 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 bg-white shadow-md rounded-md px-8 py-6 space-y-5 drop-shadow-lg dark:bg-slate-800">
            <h1 class="text-2xl font-semibold">{"Options"}</h1>

            <form class="pb-8 mb-4 h-[400px] overflow-auto">
                <div class="mb-4">
                    <ToggleSwitch label="Words" value=show_words/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Average Sentence (Words)" value=show_avg_sentence_words/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Average Word Length" value=show_avg_word_length/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Characters" value=show_characters/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Character Count (No Spaces)" value=show_character_count_no_spaces/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Line Count" value=show_line_count/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Longest Sentence (Words)" value=show_longest_sentence_words/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Paragraphs" value=show_paragraphs/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Reading Time" value=show_reading_time/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Sentences" value=show_sentences/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Shortest Sentence (Words)" value=show_shortest_sentence_words/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Speaking Time" value=show_speaking_time/>
                </div>

                <div class="mb-4">
                    <ToggleSwitch label="Unique Words" value=show_unique_words/>
                </div>
            </form>

            <div class="flex justify-end">
                <button id="done" class="px-5 py-2 bg-indigo-500 hover:bg-indigo-700 text-white cursor-pointer rounded-md" on:click=onbutton_done>
                    {"Done"}</button>
            </div>
        </div>
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GlobalState {
    pub text: RwSignal<String>,
    pub dictionary: RwSignal<HashMap<String, u32>>,
    pub statistics_options: RwSignal<Vec<StatisticOption>>,
}

impl GlobalState {
    fn new() -> Self {
        let storage = window().local_storage().unwrap().unwrap();

        let text = create_rw_signal(storage.get_item("text").unwrap().unwrap_or_default());
        let dictionary = create_rw_signal(HashMap::new());
        let statistics_options = create_rw_signal(
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
            dictionary,
            statistics_options,
        }
    }

    pub fn word_count(&self) -> usize {
        let text = self.text.get();
        let pattern = &WORD_REGEX;
        let matches = pattern.find_iter(&text);
        let words: Vec<&str> = matches.map(|m| m.as_str()).collect();

        words.len()
    }

    pub fn avg_word_count(&self) -> f64 {
        let text = self.text.get();
        let pattern = &WORD_REGEX;
        let matches = pattern.find_iter(&text);
        let words: Vec<usize> = matches.map(|m| m.as_str().len()).collect();

        words.iter().sum::<usize>() as f64 / words.len() as f64
    }

    pub fn avg_sentence_words(&self) -> f64 {
        let text = self.text.get();

        let sentences: Vec<usize> = {
            let pattern = &SENTENCE_REGEX;
            let matches = pattern.find_iter(&text);
            matches.map(|m| m.as_str().len()).collect()
        };

        let words: Vec<&str> = {
            let pattern = &WORD_REGEX;
            let matches = pattern.find_iter(&text);
            matches.map(|m| m.as_str()).collect()
        };

        words.len() as f64 / sentences.len() as f64
    }

    pub fn unique_word_count(&self) -> usize {
        let text = self.text.get();
        let pattern = &WORD_REGEX;
        let matches = pattern.find_iter(&text);
        let words: Vec<&str> = matches.map(|m| m.as_str()).collect();

        words
            .iter()
            .fold(Vec::new(), |mut acc, word| {
                if !acc.contains(word) {
                    acc.push(word)
                }

                acc
            })
            .len()
    }

    pub fn sentence_count(&self) -> usize {
        let text = self.text.get();
        let pattern = &SENTENCE_REGEX;
        let matches = pattern.find_iter(&text);
        let sentences: Vec<&str> = matches.map(|m| m.as_str()).collect();

        sentences.len()
    }

    pub fn show_longest_sentence_words_count(&self) -> usize {
        let text = self.text.get();
        let patten = &SENTENCE_REGEX;
        let matches = patten.find_iter(&text);
        let sentences: Vec<&str> = matches.map(|m| m.as_str()).collect();

        let mut sentences: Vec<usize> = sentences
            .iter()
            .map(|sentence| {
                let word_pattern = &WORD_REGEX;
                let matches = word_pattern.find_iter(sentence);
                matches.count()
            })
            .collect();

        sentences.sort();

        sentences.last().cloned().unwrap_or_default()
    }

    pub fn show_shortest_sentence_words_count(&self) -> usize {
        let text = self.text.get();
        let patten = &SENTENCE_REGEX;
        let matches = patten.find_iter(&text);
        let sentences: Vec<&str> = matches.map(|m| m.as_str()).collect();

        let mut sentences: Vec<usize> = sentences
            .iter()
            .map(|sentence| {
                let word_pattern = &WORD_REGEX;
                let matches = word_pattern.find_iter(sentence);
                matches.count()
            })
            .collect();

        sentences.sort();

        sentences.first().cloned().unwrap_or_default()
    }

    pub fn paragraph_count(&self) -> usize {
        let text = self.text.get();
        if text.is_empty() {
            return 0;
        }

        let pattern = &PARAGRAPH_REGEX;
        let paragraphs: Vec<&str> = pattern.split(&text).collect();

        paragraphs.len()
    }

    pub fn character_count_no_spaces(&self) -> usize {
        self.text.get().chars().filter(|char| *char != ' ').count()
    }

    pub fn character_total(&self) -> usize {
        self.text.get().chars().count()
    }

    pub fn line_count(&self) -> usize {
        self.text.get().lines().count()
    }
}

#[component]
pub fn App() -> impl IntoView {
    let storage = window().local_storage().unwrap().unwrap();
    let state = GlobalState::new();
    provide_context(state);

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
                let re = &WORD_REGEX;
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

    let open_options = |_: MouseEvent| {
        let dialog = document().get_element_by_id("dialog").unwrap();
        let overlay = document().get_element_by_id("overlay").unwrap();

        dialog.class_list().remove_1("hidden").unwrap();
        overlay.class_list().remove_1("hidden").unwrap();
    };

    let close_options = |_: MouseEvent| {
        let dialog = document().get_element_by_id("dialog").unwrap();
        let overlay = document().get_element_by_id("overlay").unwrap();

        dialog.class_list().add_1("hidden").unwrap();
        overlay.class_list().add_1("hidden").unwrap();
    };

    create_effect(move |_| {
        let storage = window().local_storage().unwrap().unwrap();
        let _ = state.text.get();
        get_result();

        storage
            .set_item(
                "statistics_options",
                &json!(state.statistics_options.get()).to_string(),
            )
            .unwrap();
    });

    view! {
        <main class="md:mx-auto container h-screen">

            <div id="overlay" class="fixed hidden z-40 w-screen h-screen inset-0 bg-gray-900 bg-opacity-60"></div>

            <OptionsDialog onbutton_done=close_options statistics_options=state.statistics_options />

            <div class="space-y-7">
                <div class="lg:flex bg-gray-200 p-2 mb-6 mt-auto mb-auto dark:bg-gray-800">
                    <div class="lg:flex lg:flex-col w-full">
                        <div class="flex items-center justify-between px-3 py-2 border-b dark:border-gray-600">
                            <div class="flex flex-wrap items-center divide-gray-200 sm:divide-x sm:rtl:divide-x-reverse dark:divide-gray-600">
                                <div class="flex items-center space-x-1 rtl:space-x-reverse sm:pe-4">
                                    <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600" on:click=clear_input>
                                        <svg class="w-4 h-4" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M38 12.83L35.17 10 24 21.17 12.83 10 10 12.83 21.17 24 10 35.17 12.83 38 24 26.83 35.17 38 38 35.17 26.83 24z"/><path d="M0 0h48v48H0z" fill="none"/></svg>
                                        <span class="sr-only">{ "Clear Input" }</span>
                                    </button>
                                    <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600" on:click=open_options>
                                        <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                                                <path d="M18 7.5h-.423l-.452-1.09.3-.3a1.5 1.5 0 0 0 0-2.121L16.01 2.575a1.5 1.5 0 0 0-2.121 0l-.3.3-1.089-.452V2A1.5 1.5 0 0 0 11 .5H9A1.5 1.5 0 0 0 7.5 2v.423l-1.09.452-.3-.3a1.5 1.5 0 0 0-2.121 0L2.576 3.99a1.5 1.5 0 0 0 0 2.121l.3.3L2.423 7.5H2A1.5 1.5 0 0 0 .5 9v2A1.5 1.5 0 0 0 2 12.5h.423l.452 1.09-.3.3a1.5 1.5 0 0 0 0 2.121l1.415 1.413a1.5 1.5 0 0 0 2.121 0l.3-.3 1.09.452V18A1.5 1.5 0 0 0 9 19.5h2a1.5 1.5 0 0 0 1.5-1.5v-.423l1.09-.452.3.3a1.5 1.5 0 0 0 2.121 0l1.415-1.414a1.5 1.5 0 0 0 0-2.121l-.3-.3.452-1.09H18a1.5 1.5 0 0 0 1.5-1.5V9A1.5 1.5 0 0 0 18 7.5Zm-8 6a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7Z"/>
                                            </svg>
                                        <span class="sr-only">{ "Settings" }</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="lg:flex lg:flex-row">
                            <div class="lg:w-8/12 p-2">
                                <textarea
                                class="block w-full h-96 lg:h-full p-2 mb-1 border-2 border-gray-400 rounded-lg focus:outline-none dark:bg-black"
                                placeholder="Enter text here"
                                prop:value={move || state.text.get()}
                                on:input=update_text></textarea>
                            </div>
                            <div class="lg:w-4/12 p-2">
                                {
                                    move || view! { <StatisticsOptionsPanel />}
                                }
                                <div class="bg-white p-3 rounded-md border-2 border-gray-700 dark:bg-gray-800">
                                    <div class="text-3xl mt-2 mb-4 h5">{"Keyword Density"}</div>
                                    <div class="relative overflow-auto h-full max-h-56 mb-4 border-b-2">
                                        {move || if state.text.get().is_empty() {
                                            view! {
                                                <>
                                                    <p>{"Start typing to get a list of keywords that are most used"}</p>
                                                </>
                                            }
                                        } else {
                                            view! {
                                            <>
                                                <ul>
                                                    {
                                                        move || {
                                                            let dictionary = state.dictionary.get();
                                                            let mut dictionary = dictionary.iter().collect::<Vec<_>>();
                                                            dictionary.sort_by(|a, b| a.1.cmp(b.1));
                                                            dictionary.iter().enumerate().rev().map(|(index, (key, value))| {
                                                                view! {
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
                </div>
            </div>
        </main>
    }
}
