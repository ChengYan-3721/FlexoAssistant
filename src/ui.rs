pub mod components;

use std::rc::Rc;
use floem::event::{EventListener, EventPropagation};
use crate::flexo::FlexoInfo;
use floem::kurbo::Stroke;
use floem::peniko::Color;
use floem::prelude::*;
use floem::IntoView;
use crate::ui::components::format_number;

pub fn counter_view() -> impl IntoView {
    let flexo = Rc::new(FlexoInfo::new());
    let gears = flexo.gears();
    let pitch = flexo.pitch();
    let girth = flexo.girth();
    let thickness = flexo.thickness();
    let deformation = flexo.deformation();
    let count = flexo.count();
    let before = flexo.before();
    let before_flexo = Rc::clone(&flexo);
    let after = flexo.after();
    let after_flexo = Rc::clone(&flexo);
    // let one_data = flexo.one_data();
    let data = flexo.data();
    let pitch_array: Vec<String> = ["3.175", "5"].iter().map(|x| x.to_string()).collect();
    let girth_array: Vec<String> = ["0.95", "1.14", "1.7", "2.28", "2.84", "3.94"].iter().map(|x| x.to_string()).collect();


    v_stack((
        h_stack((
            components::form_input("齿数", gears, "T", Rc::clone(&flexo)),
            components::form_select("齿距", pitch, pitch_array, "mm", Rc::clone(&flexo)),
        ))
        .style(|s| s.items_start().justify_center().gap(30)),
        h_stack((
            components::form_input("版辊周长", girth, "mm", Rc::clone(&flexo)),
            components::form_select(
                "版材厚度",
                thickness,
                girth_array,
                "mm",
                Rc::clone(&flexo)
            ),
        ))
        .style(|s| s.items_start().justify_center().gap(30)),
        h_stack((
            h_stack((
                label(move || "变形率：").style(|s| s.width(50)),
                label(move || deformation).style(|s| {
                    s.width(90)
                        .height(28)
                        .items_center()
                        .font_size(18)
                        .color(Color::RED)
                        .padding_left(5)
                        .border(Stroke::new(0.48f64))
                        .border_color(Color::RED)
                        .border_radius(5)
                }),
                label(move || "%").style(|s| s.width(30)),
            ))
            .style(|s| s.height(30).items_center().justify_center().gap(5)),
            components::form_input("模数", count, "模", Rc::clone(&flexo)),
        ))
        .style(|s| s.items_start().justify_center().gap(30)),
        h_stack((
            label(|| "").style(|s| s.width(15).font_size(14)),
            label(|| "变形前").style(|s| s.width(50).font_size(14)),
            label(|| "|").style(|s| s.width(25).justify_center().font_size(14)),
            label(|| "变形后").style(|s| s.width(50).font_size(14)),
        ))
        .style(|s| s.height(30).items_center().justify_center().gap(30)),
        h_stack((
            label(|| "自定义").style(|s| s.width(40)),
            text_input(before).on_event(EventListener::KeyUp, move |_| {
                before.set(format_number(before.get()));
                before_flexo.compute("变形前");
                EventPropagation::Stop
            }),
            text_input(after).on_event(EventListener::KeyUp, move |_| {
                after.set(format_number(after.get()));
                after_flexo.compute("变形后");
                EventPropagation::Stop
            })
        ))
        .style(|s| s.height(30).items_center().justify_center().gap(5)),
        components::form_list(data),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}