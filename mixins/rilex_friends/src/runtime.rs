use std::sync::Mutex;

pub const TOGGLE_KEY: &str = "key.keyboard.right.shift";
static GUI: Mutex<GuiState> = Mutex::new(GuiState::new());

#[derive(Default)]
pub struct GuiState {
    open: bool,
    toggle_held: bool,
}

impl GuiState {
    pub const fn new() -> Self {
        Self {
            open: false,
            toggle_held: false,
        }
    }

    pub fn key(&mut self, key: &str, pressed: bool, playing: bool) {
        if !playing {
            self.open = false;
            self.toggle_held = false;
            return;
        }
        if key == TOGGLE_KEY {
            if pressed && !self.toggle_held {
                self.open = !self.open;
            }
            self.toggle_held = pressed;
        } else if key == "key.keyboard.escape" && pressed {
            self.open = false;
        }
    }

    pub fn visible(&mut self, playing: bool) -> bool {
        if !playing {
            self.open = false;
            self.toggle_held = false;
        }
        self.open
    }
}

pub fn on_key(key: &str, pressed: bool, playing: bool) {
    GUI.lock()
        .unwrap_or_else(|e| e.into_inner())
        .key(key, pressed, playing);
}

pub fn is_open(playing: bool) -> bool {
    GUI.lock()
        .unwrap_or_else(|e| e.into_inner())
        .visible(playing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_toggles_once_per_press_and_escape_closes() {
        let mut state = GuiState::new();
        state.key(TOGGLE_KEY, false, true);
        assert!(!state.visible(true));
        state.key(TOGGLE_KEY, true, true);
        state.key(TOGGLE_KEY, true, true);
        assert!(state.visible(true));
        state.key(TOGGLE_KEY, false, true);
        state.key(TOGGLE_KEY, true, true);
        assert!(!state.visible(true));
        state.key(TOGGLE_KEY, false, true);
        state.key(TOGGLE_KEY, true, true);
        state.key("key.keyboard.escape", true, true);
        assert!(!state.visible(true));
    }

    #[test]
    fn menus_and_disabled_mod_cannot_open_or_retain_window() {
        let mut state = GuiState::new();
        state.key(TOGGLE_KEY, true, false);
        assert!(!state.visible(true));
        state.key("key.keyboard.f", true, true);
        assert!(!state.visible(true));
        state.key(TOGGLE_KEY, true, true);
        assert!(state.visible(true));
        assert!(!state.visible(false));
        assert!(!state.visible(true));
    }
}
