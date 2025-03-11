pub mod components;

use crate::flexo::FlexoInfo;
use floem::kurbo::Stroke;
use floem::peniko::Color;
use floem::prelude::*;
use std::rc::Rc;
use crate::ui::components::FlexoInput;

pub fn counter_view() -> impl IntoView {
    let flexo = Rc::new(FlexoInfo::new());

    let gears = FlexoInput::new("齿数", flexo.gears(), "T", Rc::clone(&flexo));
    let girth = FlexoInput::new("版辊周长", flexo.girth(), "mm", Rc::clone(&flexo));
    let count = FlexoInput::new("模数", flexo.count(), "", Rc::clone(&flexo));
    let before = FlexoInput::new("变形前", flexo.before(), "", Rc::clone(&flexo));
    let after = FlexoInput::new("变形后", flexo.after(), "", Rc::clone(&flexo));

    let pitch = flexo.pitch();
    let thickness = flexo.thickness();
    let deformation = flexo.deformation();
    let data = Rc::new(flexo.data());
    let use_data = create_rw_signal(Rc::clone(&data));
    let pitch_array: Vec<String> = ["3.175", "5"].iter().map(|x| x.to_string()).collect();
    let girth_array: Vec<String> = ["0.95", "1.14", "1.7", "2.28", "2.84", "3.94"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let compute = Rc::new( move |label: &str| {
        flexo.compute(label)
    });

    v_stack((
        h_stack((
            components::form_input(gears, Rc::clone(&compute)),
            components::form_select("齿距", pitch, pitch_array, "mm", Rc::clone(&compute)),
        ))
        .style(|s| s.items_start().justify_center().gap(30)),
        h_stack((
            components::form_input(girth, Rc::clone(&compute)),
            components::form_select("版材厚度", thickness, girth_array, "mm", Rc::clone(&compute)),
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
            components::form_input(count, Rc::clone(&compute)),
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
            before.input(Rc::clone(&compute)),
            after.input(Rc::clone(&compute)),
        ))
        .style(|s| s.height(30).items_center().justify_center().gap(5)),
        components::form_list(use_data.get().to_vec()),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
