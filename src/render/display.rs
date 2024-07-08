use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    draw_target::DrawTarget,
};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorDisplay;
use crate::PTouchError;

/// In memory display for drawing / rendering data
pub struct Display {
    y: usize,
    y_max: usize,
    data: Vec<Vec<u8>>,
}

impl Display {
    /// Create a new display with the provided height and minimum width
    pub fn new(y: usize, min_x: usize) -> Self {
        let mut y_max = y; //20
        while y_max % 8 != 0 {
            y_max += 1;
        } //24

        Self {
            y,
            y_max,
            data: vec![vec![0u8; y_max / 8]; min_x],
        }
    }

    /// Set a pixel value by X/Y location
    pub fn set(&mut self, x: usize, y: usize, v: bool) -> Result<(), PTouchError> {
        // Check Y bounds
        if y > self.y {
            return Err(PTouchError::RenderError);
        }

        // Extend buffer in X direction
        while x >= self.data.len() {
            self.data.push(vec![0u8; self.y_max / 8])
        }

        // Fetch pixel storage
        let c = &mut self.data[x][y / 8];

        // Update pixel
        match v {
            true => *c |= 1 << ((y % 8) as u8),
            false => *c &= !(1 << ((y % 8) as u8)),
        }

        Ok(())
    }
    
    pub fn render(&self) -> Result<Vec<Vec<u8>>, PTouchError> {
        let s = self.size();

        println!("Raster display size: {:?}", s);

        // Initialize a buffer with dimensions s.width x (s.height / 8)
        let mut buff = vec![vec![0u8; (s.height as usize + 7) / 8]; s.width as usize];

        for x in 0..(s.width as usize) {
            for y in 0..(s.height as usize) {
                let p = self.get(x, y);

                if p? {
                    buff[x][y / 8] |= 1 << (7 - (y % 8));
                }
            }
        }

        // Create a human-readable preview
        // let mut preview = String::new();
        // for y in 0..(s.height as usize) {
        //     for x in 0..(s.width as usize) {
        //         let byte = buff[x][y / 8];
        //         let bit = (byte >> (7 - (y % 8))) & 1;
        //         preview.push(if bit == 1 { '#' } else { ' ' });
        //     }
        //     preview.push('\n');
        // }
        // 
        // // Print the preview
        // print!("{}", preview);

        Ok(buff)
    }


    /// Fetch a pixel value by X/Y location
    pub fn get(&self, x: usize, y: usize) -> Result<bool, PTouchError> {
        // Check Y bounds
        if y > self.y {
            return Err(PTouchError::RenderError);
        }

        // Fetch pixel storage
        let c = self.data[x][y / 8];

        // Check bits
        Ok(c & (1 << (y % 8) as u8) != 0)
    }

    /// Fetch a pixel value by X/Y location
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<Pixel<BinaryColor>, PTouchError> {
        let v = match self.get(x, y)? {
            true => BinaryColor::On,
            false => BinaryColor::Off,
        };

        Ok(Pixel(Point::new(x as i32, y as i32), v))
    }
    pub fn size(&self) -> Size {
        Size::new(self.data.len() as u32, self.y as u32)
    }
}
/// DrawTarget impl for in-memory Display type
impl Dimensions for Display {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(self.data.len() as u32, self.y as u32))
    }
}


// Implement DrawTarget for Display
impl DrawTarget for Display {
    type Color = BinaryColor;
    type Error = PTouchError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            self.set(coord.x as usize, coord.y as usize, color.is_on())?;
        }
        Ok(())
    }
}

// Define a new trait DrawPixel
pub trait DrawPixel {
    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), PTouchError>;
}

// Implement DrawPixel for SimulatorDisplay<BinaryColor>
impl DrawPixel for SimulatorDisplay<BinaryColor> {
    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), PTouchError> {
        Ok(self.draw_iter(core::iter::once(pixel)).map_err(|_e| PTouchError::RenderError)?)
    }
}




/// test for the display
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let mut display = Display::new(8, 4);
        display.set(0, 0, true).unwrap();
        display.set(1, 0, true).unwrap();
        display.set(0, 1, true).unwrap();
        display.bounding_box();
        display.render().expect("TODO: panic message");
    }
}