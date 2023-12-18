use std::{process, rc::Rc, sync::RwLock};

use eframe::{
    egui::{self, Context, Id, RichText, Ui, ViewportCommand},
    emath::Align,
    epaint::Color32,
};

use dyn_clone::{clone_box, DynClone};

use {
    dnd_data::{character::Character, DnData},
    doice_utils::{Named, Search},
};

use super::components::DiceGrapher;

pub struct DCtx {
    focus: Option<usize>,
    changed_focus: bool,
    crnt_i: usize,
    data: Rc<AppData>,
}

impl DCtx {
    pub fn new(
        focus: Option<usize>,
        changed_focus: bool,
        crnt_i: usize,
        data: Rc<AppData>,
    ) -> Self {
        Self {
            focus,
            changed_focus,
            crnt_i,
            data,
        }
    }

    pub fn has_focus(&self) -> bool {
        self.focus.is_some_and(|i| i == self.crnt_i)
    }

    pub fn request_focus(&mut self) -> bool {
        if self.focus.is_none() {
            self.changed_focus = true;
            self.focus = Some(self.crnt_i);
            return true;
        }
        false
    }

    pub fn surrender_focus(&mut self) {
        if self.focus.is_some_and(|i| i == self.crnt_i) {
            self.focus = None;
            self.changed_focus = true;
        }
    }

    pub fn data(&self) -> &Rc<AppData> {
        &self.data
    }
}

/// Maybe this'll be used for inter-activity communication, but there probably is a better way. Currently empty.
pub struct AppData {
    pub dice_grapher: RwLock<DiceGrapher>,
    pub dnd_data: &'static DnData,
    pub character: RwLock<Character<'static>>,
}

impl AppData {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dnd_data_heap = Box::new(DnData::new());
        let dnd_data = Box::leak(dnd_data_heap);
        AppData {
            dice_grapher: RwLock::new(DiceGrapher::new(cc.egui_ctx.clone())),
            dnd_data,
            character: RwLock::new(Character::init_test(dnd_data)),
        }
    }
}

// impl Default for AppData {
//     fn default() -> Self {
//         let dnd_data_heap = Box::new(DnData::new());
//         Self { dice_grapher: Default::default(), dnd_data: Box::leak(dnd_data_heap), character: Default::default() }
//     }
// }

/// Highest level container of all the ui. Implements eframe::app to make things happen.
#[cfg(feature = "eframe")]
pub struct DoiceApp {
    taskbar: Taskbar,
    data: Rc<AppData>,
}

#[cfg(feature = "eframe")]
impl DoiceApp {
    /// Currently equal to DoiceApp::default().
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            data: Rc::new(AppData::new(cc)),
            taskbar: Default::default(),
        }
    }

    /// Add an activity to the UI
    pub fn register_activity<T: Activity + Default + 'static>(&mut self) {
        self.taskbar.register_activity::<T>();
    }

    fn context(&self, i: usize) -> DCtx {
        DCtx {
            focus: self.taskbar.current_focus,
            changed_focus: false,
            crnt_i: i,
            data: Rc::clone(&self.data),
        }
    }

    fn process_context(&mut self, ctx: DCtx) {
        // Don't do anything if nothing happened
        if !ctx.changed_focus {
            return;
        }

        if let Some(i) = ctx.focus {
            self.taskbar.refocus_on(i);
        } else {
            self.taskbar.unfocus();
        }
    }
}

#[cfg(feature = "eframe")]
impl eframe::App for DoiceApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("Big Taskbar").show(ctx, |ui| self.taskbar.show(ui, frame));

        let mut refocus = None;
        let mut d_ctx = self.context(0);
        // Render windows of activities
        for (i, e) in self.taskbar.open_activities.iter_mut().enumerate() {
            if !e.open {
                continue;
            }

            d_ctx.crnt_i = i;
            let win = egui::Window::new(e.act.name())
                .id(Id::new(&e.ident))
                .open(&mut e.closed_button)
                .resizable(e.act.resize())
                .show(ctx, |ui| {
                    ui.set_enabled(!self.taskbar.search_mode);

                    e.act.update(ui, &mut d_ctx);
                });

            // If the current window is not in focus and it has been clicked
            match self.taskbar.current_focus {
                Some(f) if f != i => {
                    if win.expect("No response from window!").response.clicked() {
                        refocus = Some(i);
                    }
                }
                _ => {}
            }
        }
        self.process_context(d_ctx);
        if let Some(i) = refocus {
            self.taskbar.refocus_on(i);
        }

        // Render search bar hits
        if self.taskbar.search_mode {
            egui::Window::new("Searchbar_Hits")
                .auto_sized()
                .anchor(egui::Align2::CENTER_CENTER, [0.0, -100.0])
                .collapsible(false)
                .title_bar(false)
                .scroll2([false, false])
                .show(ctx, |ui| self.taskbar.display_search_window(ui));
        }

        // Find closed activity, if any
        if let Some(closed) = self
            .taskbar
            .open_activities
            .iter()
            .enumerate()
            .find(|(_, e)| !e.closed_button)
            .map(|(i, _)| i)
        {
            // And close it
            self.taskbar.stop_activity(closed);
            self.taskbar.open_activities.remove(closed);
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: std::option::Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

/// Interface of an activity
pub trait Activity: DynClone + Send {
    /// Initializes the activity (optional)
    fn init(&mut self, _e_ctx: Context) {}
    /// Creates the activities' UI
    fn update(&mut self, ui: &mut egui::Ui, ctx: &mut DCtx);
    /// Returns the name displayed as the window title
    fn name(&self) -> &str;
    /// Tells the activity it has received focues
    fn focus(&mut self);
    /// Tells the activity it has lost focus
    fn lose_focus(&mut self);
    /// Whether the activity wants its window to be editable or not
    fn resize(&self) -> bool;
    /// Indicates in which category the activity thinks it belongs in
    fn category(&self) -> &str;
}

impl Named for Box<dyn Activity> {
    fn search_name(&self) -> &str {
        self.name()
    }
}

// impl<'col, 'item> Search<'col, 'item> for Vec<Box<dyn Activity>> {
//     const MAX_SCORE: f64 = 100_000.0;

//     type SearchBy = &'item str;

//     type SearchFor = &'col Box<dyn Activity>;

//     fn calculate_distances(&'col self, item: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
//         self.iter()
//             .map(|act| (strsim::jaro(item, act.name()) * Self::MAX_SCORE, act))
//     }
// }

/// Contains an open activity along with metadata
struct TaskbarEntry {
    /// False if window minimized
    open: bool,
    /// Managed by the close button in the activities' top bar
    closed_button: bool,
    /// Unique identifier for the activity
    ident: String,
    /// The activity itself
    act: Box<dyn Activity>,
}

impl Named for TaskbarEntry {
    fn search_name(&self) -> &str {
        self.act.name()
    }
}

/// Manages the creation and termination of activities
#[derive(Default)]
#[cfg(feature = "eframe")]
pub struct Taskbar {
    /// The activities that can be launched
    launchable_act: Vec<Box<dyn Activity>>,
    /// Contains the open activities along with some metadata
    open_activities: Vec<TaskbarEntry>,
    /// Buffer to store the categories
    categories: Vec<String>,
    /// Number used to differentiate different instances of the same activity
    seq: usize,
    /// The previous state of the alt button
    prev_alt: bool,
    /// Index of the activity currently in focus.
    /// Not used yet
    current_focus: Option<usize>,
    /// Is search mode engaged?
    search_mode: bool,
    /// Buffer used to store the user's search query
    search_text: String,
    /// Buffer used to store the query's result.
    /// Result consists of a score (0 = identical, higher is worse), and the index of the corresponding item
    search_res: Vec<(u32, usize)>,
}

#[cfg(feature = "eframe")]
impl Taskbar {
    /// Set the focus to the provided activity
    fn refocus_on(&mut self, open_act: usize) {
        // Let the old focus know they have been defocused
        if let Some(crnt_focus) = self.current_focus {
            self.open_activities[crnt_focus].act.lose_focus();
        }

        // Perform bounds check and assign
        self.current_focus = if open_act < self.open_activities.len() {
            self.open_activities[open_act].act.focus();
            Some(open_act)
        } else {
            None
        }
    }

    /// Remove focus from focus
    fn unfocus(&mut self) {
        if let Some(i) = self.current_focus {
            if let Some(val) = self.open_activities.get_mut(i) {
                val.act.lose_focus();
            }
        }
        self.current_focus = None;
    }

    /// Moves focus to the next non-minimized activity, unfocuses otherwise
    fn refocus(&mut self) {
        //println!("Refocusing!");
        let start = self.current_focus.unwrap_or(0) + 1;

        let new_focus = self
            .open_activities
            .iter()
            .enumerate()
            .cycle() // Makes the iterator loop around
            .skip(start) // Makes it start at the right point
            .take(self.open_activities.len() - 1) // Makes it so we only iterate over each entry once
            .find(|(_, e)| e.open) // Finds the first non-minimized activity
            .map(|(i, _)| i); // Makes it only return the index if found

        // Remove old focus
        self.unfocus();
        // Set new focus
        self.current_focus = new_focus;
        //dbg!(new_focus);
        //println!();
        // Let the activity know if needed
        if let Some(i) = self.current_focus {
            self.open_activities[i].act.focus();
        }
    }

    /// Contains all logic that must be executed before an activity is dropped
    fn stop_activity(&mut self, _act: usize) {
        // Make sure focus is surrendered
        //self.open_activities[act].act.lose_focus();
        self.refocus();
        if let Some(i) = self.current_focus {
            self.current_focus = Some(match i {
                0 => self.open_activities.len() - 1,
                _ => i - 1,
            });
        }

        // The dropping of the activity is handled elsewhere
    }

    /// Makes it so that the activity can be launched, also adds it to the ui automatigally.
    fn register_activity<T: Activity + Default + 'static>(&mut self) {
        self.launchable_act.push(Box::<T>::default());
    }

    // /// Non-pub helper function containing the logic to start a new activity
    // fn start_activity_old<T: Activity + 'static + Default + Clone>(&mut self) {
    //     // Init activity
    //     let act = T::default();
    //     // Add metadata
    //     let entry = TaskbarEntry {
    //         open: true,
    //         closed_button: true,
    //         ident: String::from(act.name()) + &self.seq.to_string(),
    //         act: Box::new(act),
    //     };
    //     // Increment seq and store entry in list
    //     self.seq += 1;
    //     self.open_activities.push(entry);
    // }

    /// Helper function to start the activity with a certain index
    fn start_activity(&mut self, i: usize, ui: &mut Ui) -> Result<(), ()> {
        // Bounds check
        if self.launchable_act.len() <= i {
            return Err(());
        }

        // Get name
        let name = self.launchable_act[i].name();

        // Add metadata
        let mut entry = TaskbarEntry {
            open: true,
            closed_button: true,
            ident: String::from(name) + &self.seq.to_string(),
            act: clone_box(self.launchable_act[i].as_ref()),
        };
        // Initialized activity
        entry.act.init(ui.ctx().clone());
        // Increment seq and store entry in list
        self.seq += 1;
        self.open_activities.push(entry);

        // Focus on new activity
        self.refocus_on(self.open_activities.len() - 1);
        Ok(())
    }

    fn display_search_res(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if self.search_res.is_empty() {
                return;
            }

            ui.label(
                RichText::new(self.launchable_act[self.search_res[0].1].name())
                    .strong()
                    .underline(),
            );
            for (_score, res) in self.search_res.iter().skip(1) {
                ui.label(self.launchable_act[*res].name());
            }
        });
    }

    /// Helper function that displays the search bar and performs the search
    fn display_search_window(&mut self, ui: &mut Ui) {
        // If tab is pressed
        if !self.open_activities.is_empty() && ui.input(|i| i.key_pressed(egui::Key::Tab)) {
            // Exit search mode
            self.search_mode = false;
            // Switch to other activity
            self.refocus();
        }

        // Do layout work
        ui.set_max_width(ui.available_width() / 2.0);

        // Create search bar and extract user input
        let search_bar = ui.text_edit_singleline(&mut self.search_text);
        if !search_bar.lost_focus() {
            search_bar.request_focus();
        }
        // If the input hasn't changed, don't bother updating the search results
        if !search_bar.changed() && !search_bar.lost_focus() {
            self.display_search_res(ui);
            return;
        }

        // Perform the search
        self.search_res.clear();
        self.search_text = self.search_text.to_lowercase();
        let query = self.search_text.trim();
        self.search_res = self.launchable_act.find_closest_matches_index(query, 20);

        // Display results
        self.display_search_res(ui);

        // If user has confirmed
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.start_activity(self.search_res[0].1, ui)
                .expect("Search result out of bounds!");
            self.search_mode = false;
            self.search_text.clear();
        }
    }

    /// Creates the entire UI
    fn show(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Handle ctrl+w shortcut
        if self.current_focus.is_some()
            && !self.open_activities.is_empty()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::W))
        {
            let i = self.current_focus.unwrap();
            //println!("Focus: {}", i);
            self.stop_activity(i);
            //println!("Removing {} !", i);
            self.open_activities.remove(i);
        }

        ui.set_height(15.0);
        egui::menu::bar(ui, |ui| {
            let crnt_alt = ui.input(|i| i.modifiers.alt);

            // Button that allows user to access the list of available activities
            let _main_but = ui.menu_button("Doice.", |ui| {
                let _list = ui.vertical(|ui| {
                    self.categories.clear();
                    let mut resp = Vec::with_capacity(self.launchable_act.len());
                    // Refill the list with a unique set of categories
                    for cat in self
                        .launchable_act
                        .iter()
                        .map(|act| String::from(act.category()))
                    {
                        // If the category is unique
                        if !self.categories.contains(&cat) {
                            // Make a submenu for it
                            ui.menu_button(&cat, |ui| {
                                // And add all activities that fall into that category
                                resp = self
                                    .launchable_act
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, act)| act.category() == cat)
                                    .map(|(i, act)| (i, ui.small_button(act.name())))
                                    .collect();
                            });
                            self.categories.push(cat);
                        }
                    }

                    // Handle clicks
                    for (clicked, _) in resp.iter().filter(|(_, r)| r.clicked()) {
                        self.start_activity(*clicked, ui)
                            .expect("There were more responses than activities, somehow.");
                    }
                });
            });

            // if main_but.response.clicked() || self.set_focus {
            //     println!("Checking for inner button...");
            //     if let Some(ref resp) = main_but.inner {
            //         println!("Focus should now be set!");
            //         resp.request_focus();
            //         self.set_focus = false;
            //     } else {
            //         println!("No inner button found.");
            //         self.set_focus = true;
            //     }

            //     dbg!(main_but);
            // }

            // Add toggle buttons to minimize activities
            for (open, act) in self
                .open_activities
                .iter_mut()
                .map(|e| (&mut e.open, &mut e.act))
            {
                let _pressed = ui.toggle_value(open, act.name());
            }

            // If alt is pressed, toggle search mode
            if crnt_alt && !self.prev_alt {
                self.search_mode = !self.search_mode;
                if !self.open_activities.is_empty() {
                    if self.search_mode && self.current_focus.is_some() {
                        self.open_activities[self.current_focus.unwrap()]
                            .act
                            .lose_focus();
                    } else if self.current_focus.is_some() {
                        self.open_activities[self.current_focus.unwrap()]
                            .act
                            .focus();
                    } else {
                        self.refocus();
                    }
                }
            }
            self.prev_alt = crnt_alt;

            // Add exit button
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                // let mut layout = LayoutJob::default();
                // layout.append("Exit", 0.0, egui::TextFormat {
                //     color: Color32::WHITE,
                //     ..Default::default()
                // });

                // Instantiate and add custom exit button
                let exit_button = egui::widgets::Button::new("Exit").fill(Color32::DARK_RED);
                let exit = ui.add(exit_button);

                // Exit on click
                if exit.clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });
    }
}
