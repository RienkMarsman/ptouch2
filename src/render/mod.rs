use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, StyledDrawable};
use embedded_graphics::text::{Alignment, Text, TextStyleBuilder};
use embedded_graphics::text::renderer::{CharacterStyle, TextRenderer};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use crate::prelude::display::{Display, DrawPixel};
use crate::PTouchError;

pub mod display;
pub mod ops;
mod text;
mod qr_code;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub orientation: Orientation,
    /// Image minimum X size
    pub min_x: usize,
    /// Image maximum X size
    pub max_x: usize,
    /// Image Y size
    pub y: usize,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            min_x: 320,
            max_x: 10 * 1024,
            y: 696,
            orientation: Orientation::Vertical,
        }
    }
}

pub struct Render {
    cfg: RenderConfig,
    display: Display,
}

impl Render {
    /// Create a new render instance
    pub fn new(cfg: RenderConfig) -> Self {
        // Setup virtual display for render data
        let display = Display::new(cfg.y as usize, cfg.min_x as usize);

        // Return new renderer
        Self { cfg, display }
    }

    pub fn render_text(&mut self, text: &str , point: Point) -> Result<(), PTouchError> {
        
        // TODO implement if text fits 
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let mut x = Text::new(text, point, character_style);
        println!("{:?}",x);
        x.draw(&mut self.display)?;
        Ok(())
    }

    pub fn show(&self) -> Result<(), PTouchError> {
        let s = self.display.size();
        println!("Display size: {:?}", s);
        // Create simulated display with dimensions based on orientation
        let mut sim_display: SimulatorDisplay<BinaryColor> = match self.cfg.orientation {
            Orientation::Vertical => SimulatorDisplay::new(s),
            Orientation::Horizontal => SimulatorDisplay::new(Size::new(s.height, s.width)),
        };

        match self.cfg.orientation {
            Orientation::Vertical => {
                // Copy buffer into simulated display horizontally
                for y in 0..s.height as usize {
                    for x in 0..s.width as usize {
                        let p = self.display.get_pixel(x, y).expect("fe");
                        sim_display.draw_pixel(p)?;
                    }
                }
            }
            Orientation::Horizontal => {
                // Copy buffer into simulated display vertically
                for y in 0..s.height as usize {
                    for x in 0..s.width as usize {
                        // Transpose the pixels correctly without mirroring
                        let p = self.display.get_pixel(x, y).expect("fe");
                        let transposed_pixel = Pixel(Point::new(y as i32, s.width as i32 - 1 - x as i32), p.1);
                        sim_display.draw_pixel(transposed_pixel)?;
                    }
                }
            }
        }

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::LcdWhite)
            .scale(1)
            .pixel_spacing(0)
            .build();

        let name = match self.cfg.orientation {
            Orientation::Vertical => format!("Label preview (Horizontal) ({}, {})", s.width, s.height),
            Orientation::Horizontal => format!("Label preview (Vertical) ({}, {})", s.height, s.width),
        };

        Window::new(&name, &output_settings).show_static(&sim_display);

        Ok(())
    }
}



