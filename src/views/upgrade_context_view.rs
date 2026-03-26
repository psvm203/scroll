use crate::utils::sycamore::{Callback, ViewVecExt};
use crate::view_models::upgrade_context_view_model::{
    Spec, UpgradeContextViewModel, spec_collection,
};
use crate::models::traces;
use sycamore::prelude::*;

mod constants {
    pub const POTENTIAL_LEGEND: &str = "확률 정보";
    pub const EQUIPMENT_LEGEND: &str = "장비 정보";
    pub const PRICE_LEGEND: &str = "시세 정보";
    pub const EQUIPMENT_DATALIST_ID: &str = "equipment-options";
}

#[component]
pub fn UpgradeContextView() -> View {
    let view_model = UpgradeContextViewModel::new();
    provide_context(view_model);

    view! { Fieldsets() }
}

#[component]
fn Fieldsets() -> View {
    view! {
        div(class="grid grid-cols-1 gap-4 md:grid-cols-2 md:gap-5 xl:grid-cols-3") {
            (fieldset(constants::POTENTIAL_LEGEND, probability_fields()))
            (fieldset(constants::EQUIPMENT_LEGEND, equipment_fields()))
            (fieldset(constants::PRICE_LEGEND, price_fields()))
        }
    }
}

fn fieldset(legend: &'static str, fields: Vec<View>) -> View {
    view! {
        fieldset(class="fieldset bg-base-200 border-base-300 rounded-box w-full min-w-0 border p-4") {
            legend(class="fieldset-legend") { (legend) }
            (fields)
        }
    }
}

fn probability_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();
    let disable_probability_inputs = view_model.is_loading.get();

    let handicraft = view_model.get_field(|context| context.handicraft);
    let enhance_mastery = view_model.get_field(|context| context.enhance_mastery);
    let upgrade_salvation = view_model.get_field(|context| context.upgrade_salvation);

    let handicraft_callback = view_model.handicraft_change_callback();
    let enhance_mastery_callback = view_model.enhance_mastery_change_callback();
    let upgrade_salvation_callback = view_model.upgrade_salvation_change_callback();

    let handicraft_tooltip = view_model.handicraft_tooltip();
    let enhance_mastery_tooltip = view_model.enhance_mastery_tooltip();
    let upgrade_salvation_tooltip = view_model.upgrade_salvation_tooltip();

    vec![
        CharacterSearch(),
        view! { div(class="divider") },
        view! {
            (field(&spec_collection::HANDICRAFT, handicraft.clone(), handicraft_callback.clone(), disable_probability_inputs))
            (handicraft_tooltip)
        },
        view! { div(class="mt-4") {
            (field(&spec_collection::ENHANCE_MASTERY, enhance_mastery.clone(), enhance_mastery_callback.clone(), disable_probability_inputs))
            (enhance_mastery_tooltip)
        }},
        view! { div(class="mt-4") {
            (field(&spec_collection::UPGRADE_SALVATION, upgrade_salvation.clone(), upgrade_salvation_callback.clone(), disable_probability_inputs))
            (upgrade_salvation_tooltip)
        }},
        view! { div(class="mt-4") { (FeverTimeToggle()) }},
    ]
}

fn equipment_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();

    let is_trace_half_price = view_model.is_trace_half_price();
    let equipment_slot = view_model.equipment_slot();
    let equipment_level = view_model.get_field(|context| context.equipment_level);
    let upgradeable_count = view_model.get_field(|context| context.upgradeable_count);
    let trace_required = view_model.get_field(|context| context.trace_required);

    let equipment_slot_callback = view_model.equipment_slot_change_callback();
    let equipment_level_callback = view_model.equipment_level_change_callback();
    let upgradeable_count_callback = view_model.upgradeable_count_change_callback();
    let trace_required_callback = view_model.trace_required_change_callback();

    vec![
        EquipmentSearch(),
        view! { div(class="divider") },
        equipment_slot_field(equipment_slot, equipment_slot_callback),
        view! { div(class="mt-4") {
            (field(
                &spec_collection::EQUIPMENT_LEVEL,
                equipment_level.clone(),
                equipment_level_callback.clone(),
                false,
            ))
        }},
        view! { div(class="mt-4") {
            (field(
                &spec_collection::UPGRADEABLE_COUNT,
                upgradeable_count.clone(),
                upgradeable_count_callback.clone(),
                false,
            ))
        }},
        view! { div(class="divider") },
        view! { div(class="mt-4") { (TraceProbabilityButtons()) }},
        view! { div(class="mt-4") {
            (trace_required_field(
                trace_required.clone(),
                trace_required_callback.clone(),
                is_trace_half_price,
            ))
        }},
        view! { div(class="mt-4") { (TraceHalfPriceToggle()) }},
    ]
}

#[component]
fn FeverTimeToggle() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let enabled = view_model.is_fever_time();
    let onclick = view_model.toggle_fever_time_callback();

    view! {
        div(class="flex items-center justify-between") {
            label(class="label p-0") { "피버타임" }
            button(
                r#type="button",
                class=if enabled { "btn btn-sm btn-primary" } else { "btn btn-sm btn-outline" },
                on:click=onclick
            ) {
                (if enabled { "ON" } else { "OFF" })
            }
        }
    }
}

#[component]
fn TraceHalfPriceToggle() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let enabled = view_model.is_trace_half_price();
    let onclick = view_model.toggle_trace_half_price_callback();

    view! {
        div(class="flex items-center justify-between") {
            label(class="label p-0") { "주문의 흔적 반값" }
            button(
                r#type="button",
                class=if enabled { "btn btn-sm btn-primary" } else { "btn btn-sm btn-outline" },
                on:click=onclick
            ) {
                (if enabled { "ON" } else { "OFF" })
            }
        }
    }
}

fn trace_required_field(value: Option<String>, callback: Callback, is_half_price: bool) -> View {
    let placeholder = spec_collection::TRACE_REQUIRED.placeholder;
    let min = spec_collection::TRACE_REQUIRED.min.to_string();
    let max = spec_collection::TRACE_REQUIRED.max.to_string();

    view! {
        label(class="label", r#for="trace-required") {
            "주문의 흔적 필요 수 "
            (if is_half_price {
                view! { span(class="text-warning font-semibold") { "(반값 적용)" } }
            } else {
                view! { span { "(반값 미적용)" } }
            })
        }
        input(
            r#type="number",
            id="trace-required",
            class="input validator w-full",
            required=true,
            placeholder=placeholder,
            value=value,
            min=min,
            max=max,
            on:change=callback,
            disabled=false
        ) {}
    }
}

fn price_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();

    let trace_price = view_model.get_field(|context| context.trace_price);
    let innocent_scroll_price = view_model.get_field(|context| context.innocent_scroll_price);
    let ark_innocent_scroll_price =
        view_model.get_field(|context| context.ark_innocent_scroll_price);
    let white_scroll_price = view_model.get_field(|context| context.white_scroll_price);

    let trace_price_callback = view_model.trace_price_change_callback();
    let innocent_scroll_price_callback = view_model.innocent_scroll_price_change_callback();
    let ark_innocent_scroll_price_callback =
        view_model.ark_innocent_scroll_price_change_callback();
    let white_scroll_price_callback = view_model.white_scroll_price_change_callback();

    let trace_price_tooltip = view_model.trace_price_tooltip();
    let innocent_scroll_price_tooltip = view_model.innocent_scroll_price_tooltip();
    let ark_innocent_scroll_price_tooltip = view_model.ark_innocent_scroll_price_tooltip();
    let white_scroll_price_tooltip = view_model.white_scroll_price_tooltip();

    [
        view! {
            (field(&spec_collection::TRACE_PRICE, trace_price.clone(), trace_price_callback.clone(), false))
            (trace_price_tooltip)
        },
        view! {
            (field(
                &spec_collection::INNOCENT_SCROLL_PRICE,
                innocent_scroll_price.clone(),
                innocent_scroll_price_callback.clone(),
                false
            ))
            (innocent_scroll_price_tooltip)
        },
        view! {
            (field(
                &spec_collection::ARK_INNOCENT_SCROLL_PRICE,
                ark_innocent_scroll_price.clone(),
                ark_innocent_scroll_price_callback.clone(),
                false
            ))
            (ark_innocent_scroll_price_tooltip)
        },
        view! {
            (field(
                &spec_collection::WHITE_SCROLL_PRICE,
                white_scroll_price.clone(),
                white_scroll_price_callback.clone(),
                false
            ))
            (white_scroll_price_tooltip)
        },
    ]
    .into_iter()
    .collect::<Vec<View>>()
    .join(|| view! { div(class="divider") })
}

fn equipment_slot_field(value: Option<String>, callback: Callback) -> View {
    let selected_weapon = value.as_deref() == Some(traces::EQUIPMENT_SLOT_WEAPON);
    let selected_armor = value.as_deref() == Some(traces::EQUIPMENT_SLOT_ARMOR);
    let selected_glove = value.as_deref() == Some(traces::EQUIPMENT_SLOT_GLOVE);
    let selected_accessory = value.as_deref() == Some(traces::EQUIPMENT_SLOT_ACCESSORY);
    let selected_heart = value.as_deref() == Some(traces::EQUIPMENT_SLOT_HEART);
    let selected_default = value.is_none();

    view! {
        label(class="label") { "장비 슬롯" }
        select(class="select w-full", on:change=callback) {
            option(value="", selected=selected_default, disabled=true) { "장비 슬롯 선택" }
            option(value=traces::EQUIPMENT_SLOT_WEAPON, selected=selected_weapon) { "무기" }
            option(value=traces::EQUIPMENT_SLOT_ARMOR, selected=selected_armor) { "방어구" }
            option(value=traces::EQUIPMENT_SLOT_GLOVE, selected=selected_glove) { "장갑" }
            option(value=traces::EQUIPMENT_SLOT_ACCESSORY, selected=selected_accessory) { "장신구" }
            option(value=traces::EQUIPMENT_SLOT_HEART, selected=selected_heart) { "기계 심장" }
        }
    }
}

#[component]
fn TraceProbabilityButtons() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let selected_probability = view_model.trace_probability();
    let actual_probability_text = view_model.actual_trace_probability_text();
    let probabilities = [100_u32, 70_u32, 30_u32, 15_u32];

    view! {
        div(class="space-y-2") {
            label(class="label") { "주문의 흔적 확률" }
            div(class="flex flex-wrap gap-2") {
                (probabilities
                    .into_iter()
                    .map(|probability| {
                        let onclick = view_model.trace_probability_change_callback(probability);
                        let is_selected = selected_probability == Some(probability);
                        let class = if is_selected { "btn btn-sm btn-primary" } else { "btn btn-sm btn-outline" };

                        view! {
                            button(
                                r#type="button",
                                class=class,
                                on:click=onclick
                            ) {
                                (format!("{probability}%"))
                            }
                        }
                    })
                    .collect::<Vec<View>>())
            }
            p(class="text-sm text-info font-semibold mt-1") { (actual_probability_text) }
        }
    }
}

#[component]
fn EquipmentSearch() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let onchange = view_model.equipment_search_callback();
    let options = view_model.equipment_search_options();

    view! {
        div(class="space-y-2") {
            label(class="label") { "장비 검색" }
            input(
                r#type="text",
                class="input w-full",
                placeholder="장비 이름 검색",
                list=constants::EQUIPMENT_DATALIST_ID,
                on:change=onchange
            ) {}
            datalist(id=constants::EQUIPMENT_DATALIST_ID) {
                (options
                    .iter()
                    .copied()
                    .map(equipment_option)
                    .collect::<Vec<View>>())
            }
        }
    }
}

fn equipment_option(value: &'static str) -> View {
    view! {
        option(value=value) {}
    }
}

#[component]
fn CharacterSearch() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let onchange = view_model.character_search_callback();
    let is_loading = view_model.is_loading.get();
    let api_error_message = view_model.api_error_message.get_clone();

    view! {
        div(class="space-y-2") {
            label(class="input w-full") {
                svg(class="h-[1em] opacity-50", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 24 24") {
                    g(stroke-linejoin="round",
                    stroke-linecap="round",
                    stroke-width="2.5",
                    fill="none",
                    stroke="currentColor") {
                        circle(cx="11", cy="11", r="8") {}
                        path(d="m21 21-4.3-4.3") {}
                    }
                }
                input(
                    r#type="search",
                    class="grow",
                    placeholder="캐릭터 닉네임 검색",
                    on:change=onchange,
                    disabled=is_loading
                ) {}
                kbd(class="kbd kbd-sm") { "↲" }
            }
            (if is_loading {
                view! { progress(class="progress progress-primary w-full") {} }
            } else {
                view! {}
            })
            (if let Some(message) = api_error_message {
                view! { p(class="text-error text-sm") { (message) } }
            } else {
                view! {}
            })
        }
    }
}

fn field(spec: &Spec, value: Option<String>, callback: Callback, disabled: bool) -> View {
    let label = spec.label;
    let placeholder = spec.placeholder;
    let min = spec.min.to_string();
    let max = spec.max.to_string();

    view! {
        label(class="label", r#for=label) { (label) }
        input(
            r#type="number",
            id=label,
            class="input validator w-full",
            required=true,
            placeholder=placeholder,
            value=value,
            min=min,
            max=max,
            on:change=callback,
            disabled=disabled
        ) {}
    }
}
