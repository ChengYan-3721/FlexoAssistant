use std::rc::Rc;
use std::sync::OnceLock;
use floem::event::{EventListener, EventPropagation};
use floem::kurbo::Stroke;
use floem::prelude::dropdown::Dropdown;
use floem::prelude::*;
use floem::style::Style;
use regex::Regex;
use crate::flexo::{FlexoData, FlexoInfo};

pub fn form_input(
    text: &'static str,
    buffer: RwSignal<String>,
    unit: &'static str,
    flexo: Rc<FlexoInfo>
) -> impl IntoView {
    h_stack((
        label(move || text).style(|s| s.width(50)),
        text_input(buffer)
            .style(|s| s.width(90))
            .on_event(EventListener::KeyUp, move |_| {
                buffer.set(format_number(buffer.get()));
                flexo.compute(text);
                EventPropagation::Stop
            }),
        label(move || unit).style(|s| s.width(30)),
    ))
    .style(|s| s.height(30).items_center().justify_center().gap(5))
}

pub fn form_select<I, T>(
    text: &'static str,
    buffer: RwSignal<T>,
    iterator: I,
    unit: &'static str,
    flexo: Rc<FlexoInfo>
) -> impl IntoView
where
    I: IntoIterator<Item = T> + Clone + 'static,
    T: Clone + std::fmt::Display + 'static,
{
    h_stack((
        label(move || text).style(|s| s.width(50)),
        Dropdown::new(move || buffer.get(), iterator)
            .on_accept(move |val| {
                buffer.set(val);
                flexo.compute(text);
            })
            .style(|s| s.width(90)),
        label(move || unit).style(|s| s.width(30)),
    ))
    .style(|s| s.height(30).items_center().justify_center().gap(5))
}
pub fn form_list(data: Vec<FlexoData>) -> impl IntoView {
    let style = || {
        Style::new()
            .width(124)
            .height(28)
            .items_center()
            .padding_left(5)
            .border(Stroke::new(0.48f64))
            .border_color(Color::GRAY)
            .border_radius(5)
    };

    let list: im::Vector<usize> = (0..data.len()).collect();
    let (list, _set_list) = create_signal(list);
    scroll({
        virtual_stack(
            VirtualDirection::Vertical,
            VirtualItemSize::Fixed(Box::new(|| 40.0)),
            move || list.get(),
            move |item| *item,
            move |i| {
                let (id, before, after) = (data[i].id, data[i].before, data[i].after);
                h_stack((
                    label(move || id).style(|s| s.width(40)),
                    label(move || before).style(move |_| style()),
                    label(move || after).style(move |_| style()),
                ))
                .style(move |s| s.height(40).items_center().justify_center().gap(5))
            },
        )
        .style(move |s| s.flex_col())
    })
    .style(move |s| s.height(400))
}

// 正则表达式在首次调用 get_regex() 时初始化
fn get_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init( || Regex::new(r"(-)?(\d*)(\.\d*)?").unwrap())
}

pub fn format_number(input: String) -> String {
    // 匹配数字部分
    let captures = get_regex().captures(input.trim()).unwrap();

    // 提取（负号）整数和小数部分
    let negative_part = captures.get(1).map(|m| m.as_str()).unwrap_or("");
    let integer_part = captures.get(2).map(|m| m.as_str()).unwrap_or("0");
    let decimal_part = captures.get(3).map(|m| m.as_str()).unwrap_or("");

    // 整数为空但有小数部分时自动在小数点前面添加0
    let integer_part = if integer_part.is_empty() && !decimal_part.is_empty() {
        "0"
    } else {
        integer_part
    };

    // 拼接完整数字字符串
    format!("{}{}{}", negative_part, integer_part, decimal_part)
}
