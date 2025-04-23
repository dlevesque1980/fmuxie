pub struct FocusManager {
    focus_index: usize,
    component_count: usize,
}

impl FocusManager {
    pub fn new(component_count: usize) -> Self {
        Self {
            focus_index: 0,
            component_count,
        }
    }

    pub fn next(&mut self) {
        self.focus_index = (self.focus_index + 1) % self.component_count;
    }

    pub fn previous(&mut self) {
        if self.focus_index == 0 {
            self.focus_index = self.component_count - 1;
        } else {
            self.focus_index -= 1;
        }
    }

    pub fn current(&self) -> usize {
        self.focus_index
    }
}