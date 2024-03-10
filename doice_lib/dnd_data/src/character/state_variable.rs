use std::fmt::Display;

use bitflags::bitflags;

use doice_utils::Named;


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateMoment: u8 {
        const LR = 0x01;
        const SR = 0x02;
        const AR = Self::LR.bits() | Self::SR.bits();
        const MANUAL = 0x04;
    }
}

/// Processes an update signal in an sv
pub type SVUpdater = fn(&StateVariable, Option<u32>, UpdateMoment) -> u32;

pub mod svupdaters {
    use super::{StateVariable, UpdateMoment};

    pub fn default(sv: &StateVariable, input: Option<u32>, _moment: UpdateMoment) -> u32 {
        if let Some(input_val) = input {
            input_val
        } else {
            sv.current
        }
    }

    pub fn regain_all(sv: &StateVariable, input: Option<u32>, moment: UpdateMoment) -> u32 {
        match moment {
            UpdateMoment::LR | UpdateMoment::SR => {
                if !sv.update_on.intersection(moment).is_empty() {
                    sv.max
                } else {
                    sv.current
                }
            }
            UpdateMoment::MANUAL => {
                if let Some(input_val) = input {
                    input_val
                } else {
                    sv.current
                }
            }
            _ => sv.current,
        }
    }

    pub fn regain_half(sv: &StateVariable, input: Option<u32>, moment: UpdateMoment) -> u32 {
        match moment {
            UpdateMoment::LR | UpdateMoment::SR => {
                if !sv.update_on.intersection(moment).is_empty() {
                    (sv.current + sv.max / 2).max(sv.max)
                } else {
                    sv.current
                }
            }
            UpdateMoment::MANUAL => {
                if let Some(input_val) = input {
                    input_val
                } else {
                    sv.current
                }
            }
            _ => sv.current,
        }
    }
}

pub struct StateVariableBuilder(StateVariable);

impl StateVariableBuilder {
    pub fn with_category(mut self, category: &str) -> StateVariableBuilder {
        self.0.name_cat = format!("{}:{}", self.0.name_cat, category);
        self
    }

    pub fn with_init_val(mut self, init_val: u32) -> StateVariableBuilder {
        self.0.current = init_val;
        self
    }

    pub fn with_max_val(mut self, max_val: u32) -> StateVariableBuilder {
        self.0.max = max_val;
        self
    }

    pub fn updateable_on(mut self, moment: UpdateMoment) -> StateVariableBuilder {
        self.0.update_on = moment;
        self
    }

    pub fn update_using(mut self, update_logic: SVUpdater) -> StateVariableBuilder {
        self.0.update_with = Some(update_logic);
        self
    }

    pub fn build(self) -> StateVariable {
        self.0
    }
}

pub struct StateVariable {
    name_cat: String,
    current: u32,
    max: u32,
    update_on: UpdateMoment,
    update_with: Option<SVUpdater>,
}

impl StateVariable {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Into<String>>(name: T) -> StateVariableBuilder {
        StateVariableBuilder(Self {
            name_cat: name.into(),
            ..Default::default()
        })
    }

    pub fn name(&self) -> &str {
        self.name_cat.split(':').last().unwrap()
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn set_current(&mut self, current: u32) {
        self.current = current;
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn update_on(&self) -> UpdateMoment {
        self.update_on
    }

    pub fn update(&mut self, moment: UpdateMoment) {
        if let Some(update_func) = self.update_with {
            self.current = (update_func)(self, None, moment);
        }
    }

    pub fn update_with_input(&mut self, moment: UpdateMoment, input: u32) {
        if let Some(update_func) = self.update_with {
            self.current = (update_func)(self, Some(input), moment);
        }
    }

    /// Returns the number that was used
    pub fn use_n(&mut self, n: u32) -> u32 {
        let used = self.current.max(n);
        self.current -= used;
        used
    }

    pub fn category(&self) -> Option<&str> {
        Some(self.name_cat.split_once(':')?.0)
    }
}

impl Display for StateVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}/{}", self.name(), self.current, self.max)
    }
}

impl Named for StateVariable {
    fn search_name(&self) -> &str {
        self.name()
    }
}

impl Default for StateVariable {
    fn default() -> Self {
        Self {
            name_cat: Default::default(),
            current: Default::default(),
            max: Default::default(),
            update_on: UpdateMoment::empty(),
            update_with: Default::default(),
        }
    }
}
