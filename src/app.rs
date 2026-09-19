#[derive(Debug, Default)]
pub struct App {
   pub selected_row: usize,
   pub current_state: String, //placeholder for enum
   pub available_tracks: Vec<String>,
   pub is_scanning: i32,
   pub should_quit: bool
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
   
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

}
