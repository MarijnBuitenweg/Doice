use std::ops::Add;

use {
    egui::epaint::{text::LayoutJob, Color32, Stroke},
    egui::TextFormat,
};

pub const LINE: Stroke = Stroke {
    width: 2.0,
    color: Color32::RED,
};

pub const LINE_ORANGE: Stroke = Stroke {
    width: 2.0,
    color: Color32::GOLD,
};

#[derive(Default, Clone)]
pub struct Layouter {
    pub sections: Vec<(String, TextFormat)>,
}

impl Layouter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append(&mut self, txt: &str) {
        self.sections
            .push((String::from(txt), TextFormat::default()));
    }

    pub fn append_colored(&mut self, txt: &str, clr: Color32) {
        self.sections.push((
            String::from(txt),
            TextFormat {
                color: clr,
                ..Default::default()
            },
        ));
    }

    pub fn append_front(&mut self, txt: &str) {
        self.sections
            .insert(0, (String::from(txt), TextFormat::default()));
    }

    pub fn append_strikethrough(&mut self, txt: &str) {
        self.sections.push((
            String::from(txt),
            TextFormat {
                strikethrough: LINE,
                ..Default::default()
            },
        ));
    }

    pub fn append_front_strikethrough(&mut self, txt: &str) {
        self.sections.insert(
            0,
            (
                String::from(txt),
                TextFormat {
                    strikethrough: LINE,
                    ..Default::default()
                },
            ),
        );
    }

    pub fn pop(&mut self) -> Option<(String, TextFormat)> {
        self.sections.pop()
    }

    pub fn remove(&mut self, index: usize) -> (String, TextFormat) {
        self.sections.remove(index)
    }
}

impl Add for Layouter {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.sections.append(&mut rhs.sections);
        self
    }
}

impl Add<&str> for Layouter {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.append(rhs);
        self
    }
}

impl From<&str> for Layouter {
    fn from(src: &str) -> Self {
        let mut out = Self::default();
        out.append(src);
        out
    }
}

impl From<Layouter> for LayoutJob {
    fn from(val: Layouter) -> Self {
        let mut out = LayoutJob::default();
        for (txt, fmt) in val.sections.into_iter() {
            out.append(&txt, 0.0, fmt);
        }
        out
    }
}
