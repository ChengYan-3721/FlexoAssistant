use floem::prelude::{RwSignal, SignalGet, SignalUpdate};
use std::collections::HashMap;

#[derive(Clone)]
pub struct FlexoData {
    pub id: RwSignal<String>,
    pub before: RwSignal<String>,
    pub after: RwSignal<String>,
}
impl FlexoData {
    pub fn new(id: RwSignal<String>, before: RwSignal<String>, after: RwSignal<String>) -> Self {
        Self { id, before, after }
    }
}

pub struct FlexoInfo {
    gears: RwSignal<String>,     // 齿数
    pitch: RwSignal<String>,     // 齿距
    girth: RwSignal<String>,     // 周长
    thickness: RwSignal<String>, // 厚度
    k_map: HashMap<&'static str, f32>,
    deformation: RwSignal<String>, // 变形率
    count: RwSignal<String>,       // 模数
    before: RwSignal<String>,      // 变形前
    after: RwSignal<String>,       // 变形后
    one_data: Vec<FlexoData>,      // 指定模数均分数据
    data: Vec<FlexoData>,          // 1-10模均分数据
    compute_girth: RwSignal<bool>, // 求周长
    compute_after: RwSignal<bool>, // 求变形后
}
impl FlexoInfo {
    pub fn new() -> Self {
        let k_list = vec![
            ("1.14", 6.06),
            ("1.7", 9.89),
            ("2.28", 13.52),
            ("2.54", 16.05),
            ("2.84", 17.04),
            ("3.94", 23.94),
            ("0.95", 5.4),
        ];
        let mut data = Vec::new();
        for i in 1..=10 {
            data.push(FlexoData::new(
                RwSignal::new(i.to_string() + "模"),
                RwSignal::new(String::new()),
                RwSignal::new(String::new()),
            ));
        }
        Self {
            gears: RwSignal::new(String::new()),
            pitch: RwSignal::new("3.175".to_string()),
            girth: RwSignal::new(String::new()),
            thickness: RwSignal::new("1.7".to_string()),
            k_map: k_list.into_iter().collect(),
            deformation: RwSignal::new(String::new()),
            count: RwSignal::new(String::new()),
            before: RwSignal::new(String::new()),
            after: RwSignal::new(String::new()),
            one_data: vec![FlexoData::new(
                RwSignal::new(String::from("n模")),
                RwSignal::new(String::new()),
                RwSignal::new(String::new()),
            )],
            data,
            compute_girth: RwSignal::new(true),
            compute_after: RwSignal::new(true),
        }
    }
    pub fn gears(&self) -> RwSignal<String> {
        self.gears
    }
    pub fn pitch(&self) -> RwSignal<String> {
        self.pitch
    }
    pub fn girth(&self) -> RwSignal<String> {
        self.girth
    }
    pub fn thickness(&self) -> RwSignal<String> {
        self.thickness
    }
    pub fn deformation(&self) -> RwSignal<String> {
        self.deformation
    }
    pub fn count(&self) -> RwSignal<String> {
        self.count
    }
    pub fn before(&self) -> RwSignal<String> {
        self.before
    }
    pub fn after(&self) -> RwSignal<String> {
        self.after
    }
    pub fn one_data(&self) -> Vec<FlexoData> {
        self.one_data.clone()
    }
    pub fn data(&self) -> Vec<FlexoData> {
        self.data.clone()
    }
    fn compute_girth(&self) {
        let gears = self.gears.get().parse::<f32>().unwrap_or(0.0);
        if gears == 0.0 {
            self.girth.set("".to_string());
        } else {
            let pitch = self.pitch.get().parse::<f32>().unwrap_or(3.175);
            self.girth.set((gears * pitch).to_string());
        }
        self.compute_deformation()
    }
    fn compute_gears(&self) {
        let girth = self.girth.get().parse::<f32>().unwrap_or(0.0);
        if girth == 0.0 {
            return self.gears.set("".to_string());
        }
        let pitch = self.pitch.get().parse::<f32>().unwrap_or(3.175);
        self.gears.set((girth / pitch).to_string());
    }
    fn compute_deformation(&self) {
        let thickness = self.thickness.get();
        let k = self.k_map.get(thickness.trim()).copied().unwrap_or(6.06);
        let girth = self.girth.get().parse::<f32>().unwrap_or(0.0);
        if girth == 0.0 {
            return self.deformation.set("".to_string());
        }
        self.deformation.set(((1.0 - k / girth) * 100.0).to_string());
    }
    pub fn compute_after(&self) {
        let before = self.before.get().parse::<f32>().unwrap_or(0.0);
        if before == 0.0 {
            return self.after.set("".to_string());
        }
        let deformation = self.deformation.get().parse::<f32>().unwrap_or(100.0);
        self.after.set((before * deformation / 100.0).to_string());
    }
    pub fn compute_before(&self) {
        let after = self.after.get().parse::<f32>().unwrap_or(0.0);
        if after == 0.0 {
            return self.before.set("".to_string());
        }
        let deformation = self.deformation.get().parse::<f32>().unwrap_or(100.0);
        self.before.set((after / deformation * 100.0).to_string());
    }
    pub fn compute(&self, label: &str) {
        match label {
            "齿数" => {
                self.compute_girth.set(true);
                self.compute_girth()
            }
            "齿距" => {
                if self.compute_girth.get() {
                    self.compute_girth();
                } else {
                    return self.compute_gears()
                }
            }
            "版辊周长" => {
                self.compute_girth.set(false);
                self.compute_gears();
                self.compute_deformation();
            }
            "版材厚度" => { self.compute_deformation() },
            "变形前" => {
                self.compute_after.set(true);
                return self.compute_after()
            }
            "变形后" => {
                self.compute_after.set(false);
                return self.compute_before()
            }
            &_ => {}
        }
        if self.compute_after.get() {
            self.compute_after()
        } else {
            self.compute_before()
        }
    }
}
