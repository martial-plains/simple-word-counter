use std::time::Duration;

use leptos::{
    component, create_effect, create_node_ref, event_target_checked, html::Input, use_context,
    view, IntoView, RwSignal, SignalGet, SignalSet,
};

use crate::{
    app::{GlobalState, StatisticOption},
    utils::{calculate_duration, paragraph_count, sentence_count},
};

#[component]
pub fn statistics_options_panel() -> impl IntoView {
    let state = use_context::<GlobalState>().unwrap_or_default();
    view! {
        <div class="mb-4 bg-white p-3 rounded-md border-2 border-gray-700 text-gray-500 dark:bg-gray-800">
            <div class="h2 text-3xl text-black mt-2 mb-4 dark:text-white">{"Statistics"}</div>
            {
                move || {
                    let mut row = Vec::new();
                    let mut rows = Vec::new();


                    for (index, option) in state.statistics_options.get().iter().enumerate() {
                        if index % 2 == 0 {
                            rows.push(view! {
                                <div class="border-b-2 border-gray-700 flex justify-between mb-4">
                                    {row.clone()}
                                </div>
                            });
                            row.clear()
                        }

                        row.push(view! {
                            <div class="w-2/5">
                                {
                                    match option {
                                        StatisticOption::Characters => view! {
                                            <>
                                                <div class="uppercase text-xs">{"Characters"}</div>
                                                <span class="text-4xl text-black dark:text-white">{state.character_total()}</span>
                                            </>
                                        },
                                        StatisticOption::Paragraphs => view! {
                                            <>
                                                <div class="uppercase text-xs">{"Paragraphs"}</div>
                                                <span class="text-4xl text-black dark:text-white">{paragraph_count(&state.text.get())}</span>
                                            </>
                                        },
                                        StatisticOption::ReadingTime => view! {
                                            <>
                                                <div class="uppercase text-xs whitespace-nowrap">
                                                    {"Reading Time"}
                                                    <span title="Based on 275 words per minute" class="inline-block">
                                                        <i class="fa-solid fa-circle-question"></i>
                                                    </span>
                                                </div>
                                                <div class="flex flex-nowrap">
                                                    <span class="text-3xl text-black dark:text-white">{move || format_duration( calculate_duration(state.word_total(), 275))}</span>
                                                </div>
                                            </>
                                        },
                                        StatisticOption::Sentences => view! {
                                            <>
                                                <div class="uppercase text-xs">{"Sentences"}</div>
                                                <span class="text-4xl text-black dark:text-white">{sentence_count(&state.text.get())}</span>
                                            </>
                                        },
                                        StatisticOption::SpeakingTime => view! {
                                            <>
                                                <div class="uppercase text-xs whitespace-nowrap">
                                                    {"Speaking Time"}
                                                    <span title="Based on 180 words per minute" class="inline-block">
                                                        <i class="fa-solid fa-circle-question"></i>
                                                    </span>
                                                </div>
                                                <div class="flex flex-nowrap">
                                                    <span class="text-3xl text-black dark:text-white">{move || format_duration( calculate_duration(state.word_total(), 180))}</span>
                                                </div>
                                            </>
                                        },
                                        StatisticOption::Words => view! {
                                            <>
                                                <div class="uppercase text-xs">{"Words"}</div>
                                                <span class="text-4xl text-black dark:text-white">{state.word_total()}</span>
                                            </>
                                        },
                                    }
                                }
                            </div>
                        });

                        if index == state.statistics_options.get().len() - 1 && !row.is_empty() {
                            rows.push(view! {
                                <div class="    border-gray-700 flex justify-between mb-2">
                                    {row.clone()}
                                </div>
                            });
                        }
                    }

                    rows
                }
            }
            <div class="border-b-2 border-gray-700 flex justify-between mb-4">
                <div class="w-2/5">

                </div>
                <div class="w-2/5">

                </div>
            </div>
        </div>
    }
}

#[component]
pub fn toggle_switch(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let onchange = move |e| value.set(event_target_checked(&e));

    create_effect(move |_| {
        if let Some(inputs) = input_ref.clone().get() {
            println!("{}", value.get());
            inputs.set_checked(value.get())
        }
    });

    view! {
        <label class="relative flex justify-between items-center group p-2 text-xl">
            {label}
            <input _ref=input_ref type="checkbox" class="absolute left-1/2 -translate-x-1/2 w-full h-full peer appearance-none rounded-md" checked=value.get() on:change=onchange />
            <span class="w-16 h-10 flex items-center flex-shrink-0 ml-4 p-1 bg-gray-300 rounded-full duration-300 ease-in-out peer-checked:bg-green-400 after:w-8 after:h-8 after:bg-white after:rounded-full after:shadow-md after:duration-300 peer-checked:after:translate-x-6 group-hover:after:translate-x-1"></span>
        </label>
    }
}

fn format_duration(duration: Duration) -> impl IntoView {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes > 60 {
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;

        view! {
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
        view! {
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
        view! {
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
