use bevy::prelude::*;
use bevy_tweening::*;

pub struct BackgroundColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let r = self.start.r() + (self.end.r() - self.start.r()) * ratio;
        let g = self.start.g() + (self.end.g() - self.start.g()) * ratio;
        let b = self.start.b() + (self.end.b() - self.start.b()) * ratio;
        let a = self.start.a() + (self.end.a() - self.start.a()) * ratio;
        *target = Color::rgba(r, g, b, a).into();
    }
}

#[derive(Debug, Clone)]
pub struct TextLens {
    pub text: String,
    pub section: usize,
}

impl TextLens {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            section: 0,
        }
    }

    pub fn with_section(self, section: usize) -> Self {
        Self {
            text: self.text,
            section,
        }
    }
}

impl Lens<Text> for TextLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        target.sections[self.section].value =
            self.text[0..(self.text.len() as f32 * ratio).floor() as usize].to_string();
    }
}
