use raylib::prelude::*;

#[derive(Clone)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    pub wait_after: f32,
    pub sound_effect: Option<String>,
}

pub struct DialogueSystem {
    lines: Vec<DialogueLine>,
    current_line: usize,
    line_start_time: f64,
    dialogue_started: bool,
    font: Font,
}

impl DialogueSystem {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let font = rl.load_font(thread, "src/assets/tiny.ttf")?;

        let lines = vec![
            DialogueLine {
                speaker: "???".to_string(),
                text: "Again... so soon?".to_string(),
                wait_after: 3.5,
                sound_effect: Some("drip".to_string()),
            },
            DialogueLine {
                speaker: "???".to_string(),
                text: "You claw your way back through the dark,\nlifetime after lifetime..."
                    .to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Tell me - don't you ever tire of this?".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "You've put me through hell!".to_string(),
                wait_after: 3.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Don't you ever get tired of watching\nme suffer?!".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Hell?".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "You speak as though it wasn't you\nwho begged for this.".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Immortality… the gift you wanted most.".to_string(),
                wait_after: 3.5,
                sound_effect: Some("laugh".to_string()),
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "And this is how you thank me?".to_string(),
                wait_after: 3.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Give me my life back!".to_string(),
                wait_after: 3.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Your life?".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "You threw it away long ago.".to_string(),
                wait_after: 3.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "What you live now is eternity.".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Immortality is meaningless\nwithout purpose.".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "You".to_string(),
                text: "Without an end,\nit's just another prison…".to_string(),
                wait_after: 4.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Ah… purpose.".to_string(),
                wait_after: 2.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Such a fragile word.".to_string(),
                wait_after: 3.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Very well, old friend…".to_string(),
                wait_after: 3.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Shall I take back my gift?".to_string(),
                wait_after: 3.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Shall I let you crumble into dust at last?".to_string(),
                wait_after: 4.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "Or will you stay… crawling through the dark\nfor another thousand years…"
                    .to_string(),
                wait_after: 4.5,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "Devil".to_string(),
                text: "…chasing my shadow?".to_string(),
                wait_after: 5.0,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "".to_string(),
                text: "What will it be, old friend?".to_string(),
                wait_after: 8.0,
                sound_effect: None,
            },
        ];

        Ok(Self {
            lines,
            current_line: 0,
            line_start_time: 0.0,
            dialogue_started: false,
            font,
        })
    }

    pub fn start(&mut self, current_time: f64) {
        self.dialogue_started = true;
        self.line_start_time = current_time;
        self.current_line = 0;
    }

    pub fn update(&mut self, current_time: f64) -> Option<String> {
        if !self.dialogue_started || self.current_line >= self.lines.len() {
            return None;
        }

        let elapsed = current_time - self.line_start_time;
        let current = &self.lines[self.current_line];

        if elapsed >= current.wait_after as f64 {
            let sound_effect = current.sound_effect.clone();
            self.current_line += 1;
            self.line_start_time = current_time;
            return sound_effect;
        }

        None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32) {
        if !self.dialogue_started || self.current_line >= self.lines.len() {
            return;
        }

        let current = &self.lines[self.current_line];
        let font_size = 20.0;
        let spacing = 2.0;
        let padding = 30;
        let max_line_width = (screen_width / 2) - padding * 2; // Maximum width per line

        // Determine color and position based on speaker
        let (color, alignment) = match current.speaker.as_str() {
            "Devil" | "???" => (Color::new(220, 80, 80, 255), "left"),
            "You" => (Color::new(80, 140, 220, 255), "right"),
            _ => (Color::new(200, 200, 200, 255), "center"),
        };

        let y_pos = screen_height - padding - 60;

        // Draw speaker name if present
        if !current.speaker.is_empty() {
            let speaker_text = format!("{}:", current.speaker);
            let speaker_size = 16;

            let text_width = d.measure_text(&speaker_text, speaker_size) as f32;
            let speaker_x = match alignment {
                "right" => (screen_width - padding) as f32 - text_width,
                "center" => (screen_width / 2) as f32 - text_width / 2.0,
                _ => padding as f32,
            };

            d.draw_text_ex(
                &self.font,
                &speaker_text,
                Vector2::new(speaker_x, y_pos as f32 - 25.0),
                speaker_size as f32,
                spacing,
                Color::new(color.r, color.g, color.b, 180),
            );
        }

        // Split and wrap text to fit within max_line_width
        let wrapped_lines = self.wrap_text(&current.text, d, max_line_width, font_size as i32);

        for (i, line) in wrapped_lines.iter().enumerate() {
            let text_width = d.measure_text(line, font_size as i32) as f32;
            let text_x = match alignment {
                "right" => {
                    // For right alignment, ensure text doesn't go off screen
                    let desired_x = (screen_width - padding) as f32 - text_width;
                    desired_x.max(padding as f32) // Don't go past left padding
                }
                "center" => (screen_width / 2) as f32 - text_width / 2.0,
                _ => padding as f32,
            };

            d.draw_text_ex(
                &self.font,
                line,
                Vector2::new(text_x, y_pos as f32 + (i as f32 * (font_size + 5.0))),
                font_size,
                spacing,
                color,
            );
        }
    }

    fn wrap_text(
        &self,
        text: &str,
        d: &mut RaylibDrawHandle,
        max_width: i32,
        font_size: i32,
    ) -> Vec<String> {
        let mut result = Vec::new();

        // First split by explicit newlines
        for paragraph in text.split('\n') {
            let paragraph = paragraph.trim();
            if paragraph.is_empty() {
                continue;
            }

            let width = d.measure_text(paragraph, font_size);

            // If line fits, add it as is
            if width <= max_width {
                result.push(paragraph.to_string());
                continue;
            }

            // Otherwise, wrap it
            let words: Vec<&str> = paragraph.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                let test_width = d.measure_text(&test_line, font_size);

                if test_width <= max_width {
                    current_line = test_line;
                } else {
                    // Current line is full, save it and start new line
                    if !current_line.is_empty() {
                        result.push(current_line);
                    }
                    current_line = word.to_string();
                }
            }

            // Add the last line
            if !current_line.is_empty() {
                result.push(current_line);
            }
        }

        result
    }

    pub fn is_finished(&self) -> bool {
        self.current_line >= self.lines.len()
    }
}
