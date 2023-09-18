use std::ops::{Add, AddAssign};

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

/// Makes it slightly easier to manipulate egui-compatible formatted text
#[derive(Default, Clone)]
pub struct Layouter {
    pub sections: Vec<(String, TextFormat)>,
}

impl Layouter {
    /// Delegates to `Layouter::default`, thereby creating an empty layouter
    #[must_use]
    pub fn new() -> Self {
        Layouter::default()
    }

    /// Adds a str with default formatting to the end
    pub fn append(&mut self, txt: &str) {
        self.sections
            .push((String::from(txt), TextFormat::default()));
    }

    /// Adds a str with some color to the end
    pub fn append_colored(&mut self, txt: &str, clr: Color32) {
        self.sections.push((
            String::from(txt),
            TextFormat {
                color: clr,
                ..Default::default()
            },
        ));
    }

    /// Adds a str with default formatting to the front
    pub fn append_front(&mut self, txt: &str) {
        self.sections
            .insert(0, (String::from(txt), TextFormat::default()));
    }

    /// Adds a str with strikethrough applied to the end
    pub fn append_strikethrough(&mut self, txt: &str) {
        self.sections.push((
            String::from(txt),
            TextFormat {
                strikethrough: LINE,
                ..Default::default()
            },
        ));
    }
    /// Adds a str with strikethrough applied to the front
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

    /// See `Vec::pop`
    pub fn pop(&mut self) -> Option<(String, TextFormat)> {
        self.sections.pop()
    }

    /// See `Vec::remove`
    pub fn remove(&mut self, index: usize) -> (String, TextFormat) {
        self.sections.remove(index)
    }
}

impl Add for Layouter {
    type Output = Self;

    /// Appends rhs to lhs and returns the result
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.sections.append(&mut rhs.sections);
        self
    }
}

impl AddAssign for Layouter {
    /// Appends rhs to lhs
    fn add_assign(&mut self, mut rhs: Self) {
        self.sections.append(&mut rhs.sections);
    }
}

impl Add<&str> for Layouter {
    type Output = Self;

    /// Appends rhs to lhs with default formatting, then returns lhs
    fn add(mut self, rhs: &str) -> Self::Output {
        self.append(rhs);
        self
    }
}

impl AddAssign<&str> for Layouter {
    fn add_assign(&mut self, rhs: &str) {
        self.append(rhs);
    }
}

impl From<&str> for Layouter {
    /// Creates `Layouter` with default formatting from str
    fn from(src: &str) -> Self {
        let mut out = Self::default();
        out.append(src);
        out
    }
}

impl From<String> for Layouter {
    fn from(value: String) -> Self {
        Layouter {
            sections: vec![(value, TextFormat::default())]
        }
    }
}

impl From<Layouter> for LayoutJob {
    fn from(val: Layouter) -> Self {
        let mut out = LayoutJob::default();
        for (txt, fmt) in val.sections {
            out.append(&txt, 0.0, fmt);
        }
        out
    }
}
