use raylib::prelude::*;

pub struct DialogueSystem {
    pub lines: Vec<DialogueLine>,
    pub current_line: usize,
    pub char_index: usize,
    pub time_since_last_char: f32,
    pub chars_per_second: f32,
    pub is_active: bool,
    pub is_complete: bool,
    pub wait_time: f32,
    pub current_wait: f32,
}

pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    pub wait_after: f32,
    pub sound_effect: Option<String>, // dripping, laugh, etc.
}

impl DialogueSystem {
    pub fn new() -> Self {
        let lines = vec![
            DialogueLine {
                speaker: "???".to_string(),
                text: "Again... so soon?".to_string(),
                wait_after: 2.0,
                sound_effect: Some("drip".to_string()),
            },
            DialogueLine {
                speaker: "???".to_string(),
                text: "You claw your way back through the dark,\nlifetime after lifetime..."
                    .to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "???".to_string(),
                text: "Tell me - don't you ever tire of this?".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "You've put me through hell!".to_string(),
                wait_after: 1.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Don't you ever get tired of watching me suffer?!".to_string(),
                wait_after: 2.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Hell?".to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "You speak as though it wasn't you\nwho begged for this.".to_string(),
                wait_after: 2.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Immortality… the gift you wanted most.".to_string(),
                wait_after: 1.5,
                sound_effect: Some("laugh".to_string()),
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "And this is how you thank me?".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Give me my life back!".to_string(),
                wait_after: 2.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Your life?".to_string(),
                wait_after: 1.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "You threw it away long ago.".to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "What you live now is eternity.".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Immortality is meaningless without purpose.".to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Without an end, it's just another prison…".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Ah… purpose.".to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Such a fragile word.".to_string(),
                wait_after: 2.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Very well, old friend…".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Shall I take back my gift?".to_string(),
                wait_after: 1.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Shall I let you crumble into dust at last?".to_string(),
                wait_after: 3.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Or will you stay… crawling through the dark\nfor another thousand years…"
                    .to_string(),
                wait_after: 2.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "…chasing my shadow?".to_string(),
                wait_after: 3.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "".to_string(),
                text: "What will it be, old friend?".to_string(),
                wait_after: 5.0,
                sound_effect: None,
            },
        ];

        Self {
            lines,
            current_line: 0,
            char_index: 0,
            time_since_last_char: 0.0,
            chars_per_second: 25.0,
            is_active: false,
            is_complete: false,
            wait_time: 0.0,
            current_wait: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.current_line = 0;
        self.char_index = 0;
        self.time_since_last_char = 0.0;
        self.is_complete = false;
        self.wait_time = 0.0;
        self.current_wait = 0.0;
    }

    pub fn update(&mut self, delta_time: f32, rl: &RaylibHandle) -> Option<String> {
        if !self.is_active || self.is_complete {
            return None;
        }

        // Handle waiting between lines
        if self.wait_time > 0.0 {
            self.current_wait += delta_time;
            if self.current_wait >= self.wait_time {
                self.wait_time = 0.0;
                self.current_wait = 0.0;
                self.current_line += 1;
                self.char_index = 0;

                if self.current_line >= self.lines.len() {
                    self.is_complete = true;
                    return None;
                }
            }
            return None;
        }

        let current = &self.lines[self.current_line];
        let char_count = current.text.chars().count();

        // Type out characters
        if self.char_index < char_count {
            self.time_since_last_char += delta_time;
            let delay = 1.0 / self.chars_per_second;

            // Skip input to complete current line (only check when typing)
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.char_index = char_count;
            } else if self.time_since_last_char >= delay {
                self.char_index += 1;
                self.time_since_last_char = 0.0;
            }
        } else {
            // Line complete, check for advance input
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.wait_time = 0.01; // Skip wait
                return current.sound_effect.clone();
            }
            // Line complete, start waiting
            if self.wait_time == 0.0 {
                self.wait_time = current.wait_after;
                return current.sound_effect.clone();
            }
        }

        None
    }

    pub fn draw<'a>(&self, d: &mut RaylibDrawHandle<'a>) {
        if !self.is_active || self.is_complete {
            return;
        }

        let current = &self.lines[self.current_line];
        // Use char indices instead of byte indices for UTF-8 safety
        let visible_text: String = current.text.chars().take(self.char_index).collect();

        // Calculate positions
        let box_width = 600;
        let box_height = 120;
        let x = (d.get_screen_width() - box_width) / 2;
        let y = d.get_screen_height() - box_height - 40;

        // Draw background box with shadow
        d.draw_rectangle(
            x + 4,
            y + 4,
            box_width,
            box_height,
            Color::new(0, 0, 0, 120),
        );
        d.draw_rectangle(x, y, box_width, box_height, Color::new(10, 10, 15, 230));
        d.draw_rectangle_lines(x, y, box_width, box_height, Color::new(80, 60, 40, 200));

        // Draw speaker name
        if !current.speaker.is_empty() {
            let speaker_color = match current.speaker.as_str() {
                "Devil" => Color::new(200, 50, 50, 255),
                "You" => Color::new(150, 150, 200, 255),
                _ => Color::new(150, 150, 150, 255),
            };

            d.draw_text(&current.speaker, x + 20, y + 15, 18, speaker_color);
        }

        // Draw dialogue text
        d.draw_text(
            &visible_text,
            x + 20,
            y + 45,
            16,
            Color::new(230, 230, 220, 255),
        );

        // Draw continue indicator when line is complete
        let char_count = current.text.chars().count();
        if self.char_index >= char_count && self.wait_time > 0.0 {
            let blink = ((d.get_time() * 3.0).sin() * 0.5 + 0.5) * 255.0;
            d.draw_text(
                "▼",
                x + box_width - 30,
                y + box_height - 25,
                20,
                Color::new(200, 200, 180, blink as u8),
            );
        }
    }
}
