use crate::flexo::{FlexoData, FlexoInfo};
use floem::event::{EventListener, EventPropagation};
use floem::kurbo::Stroke;
use floem::prelude::dropdown::Dropdown;
use floem::prelude::*;
use floem::style::Style;
use std::rc::Rc;
use std::sync::OnceLock;
use floem::ViewId;
use regex::Regex;


pub struct FlexoInput {
    label: &'static str,
    buffer: RwSignal<String>,
    unit: &'static str,
    input: TextInput,
    vid: ViewId,
    flexo: Rc<FlexoInfo>,
}
impl FlexoInput {
    pub fn new(label: &'static str, buffer: RwSignal<String>, unit: &'static str, flexo: Rc<FlexoInfo>) -> Self {
        let mut input = text_input(buffer);
        let vid = input.id();
        Self {
            label,
            buffer,
            unit,
            input,
            vid,
            flexo
        }
    }
    pub fn id(&self) -> ViewId {
        self.vid
    }
    pub fn label(&self) -> &'static str {
        self.label
    }
    pub fn unit(&self) -> &'static str {
        self.unit
    }
    pub fn buffer(&self) -> RwSignal<String> {
        self.buffer
    }
    pub fn input(self, callback: Rc<impl Fn(&str) + 'static>) -> TextInput {
        self.input.on_event(EventListener::KeyUp, move|_| {
            self.buffer.set(format_float(self.buffer.get()));
            callback(self.label);
            EventPropagation::Stop
        })
    }
}

pub fn form_input(
    flexo_input: FlexoInput,
    callback: Rc<impl Fn(&str) + 'static>
) -> impl IntoView {
    let unit = flexo_input.unit;
    h_stack((
        label(move || flexo_input.label).style(|s| s.width(50)),
        flexo_input.input(callback).style(|s| s.width(90)),
        label(move || unit).style(|s| s.width(30)),
    ))
    .style(|s| s.height(30).items_center().justify_center().gap(5))
}

pub fn form_select<I, T>(
    text: &'static str,
    buffer: RwSignal<T>,
    iterator: I,
    unit: &'static str,
    callback: Rc<impl Fn(&str) + 'static>
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
                callback(text);
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
fn get_regex_float() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(-)?0*(\d*)(\.\d*)?").unwrap())
}

pub fn format_float(input: String) -> String {
    // 匹配数字部分
    let captures = get_regex_float().captures(input.trim()).unwrap();

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

// 正则表达式在首次调用 get_regex() 时初始化
fn get_regex_int() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"0*(\d*)?").unwrap())
}
pub fn format_int(input: String) -> String {
    // 匹配数字部分
    let captures = get_regex_int().captures(input.trim()).unwrap();

    // 提取整数
    captures
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("")
        .to_string()
}