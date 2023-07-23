const NUM_KEYS: usize = 16;

pub struct Keypad {
    pub keys: [bool; NUM_KEYS],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn key_down(&mut self, idx: usize) {
        self.keys[idx] = true;
    }

    pub fn key_up(&mut self, idx: usize) {
        self.keys[idx] = false;
    }

    pub fn is_pressed(&self, idx: usize) -> bool {
        self.keys[idx]
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::keypad::Keypad;

    #[test]
    fn test_default_key_status() {
        let keypad = Keypad::new();
        assert_eq!(keypad.is_pressed(1), false);
    }

    #[test]
    fn test_press_key() {
        let mut keypad = Keypad::new();
        keypad.key_down(1);
        assert_eq!(keypad.is_pressed(1), true);
    }

    #[test]
    fn test_release_key() {
        let mut keypad = Keypad::new();
        keypad.key_down(1);
        assert_eq!(keypad.is_pressed(1), true);
        keypad.key_up(1);
        assert_eq!(keypad.is_pressed(1), false);
    }
}