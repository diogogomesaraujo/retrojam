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

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32) {
        if !self.dialogue_started || self.current_line >= self.lines.len() {
            return;
        }

        let current = &self.lines[self.current_line];

        // Font sizing and spacing
        let base_dim = screen_width.min(screen_height) as f32;
        let font_size = ((base_dim * 0.035) * 1.5).clamp(24.0, 64.0);
        let spacing = font_size * 0.1;
        let padding_x = screen_width as f32 * 0.04;
        let padding_y = screen_height as f32 * 0.05;
        let line_spacing = font_size * 0.25;

        // Color per speaker
        let color = match current.speaker.as_str() {
            "Devil" | "???" => Color::new(251, 73, 52, 255),
            "You" => Color::new(235, 219, 178, 255),
            _ => Color::new(168, 153, 132, 255),
        };

        // Split into lines
        let lines: Vec<&str> = current.text.split('\n').collect();

        // Compute total height so it sits neatly above bottom padding
        let total_text_height = lines.len() as f32 * (font_size + line_spacing) - line_spacing;
        let y_start = screen_height as f32 - padding_y - total_text_height;

        // Draw each line left-aligned
        for (i, line) in lines.iter().enumerate() {
            let width = d.measure_text(line, font_size as i32) as f32
                + spacing * (line.chars().count().saturating_sub(1)) as f32;

            let height = font_size + line_spacing;
            let x_pos = padding_x; // always left aligned
            let y_pos = y_start + (i as f32 * height);

            d.draw_text_ex(
                &self.font,
                line,
                Vector2::new(x_pos, y_pos),
                font_size,
                spacing,
                color,
            );
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_line >= self.lines.len()
    }
}
