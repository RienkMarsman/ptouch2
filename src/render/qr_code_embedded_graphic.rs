use crate::render::qr_code::QrCode;
use embedded_graphics::{
    self,
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, Point, PointsIter, Size},
    primitives::{rectangle::Points, Rectangle},
    Drawable, Pixel,
};
use embedded_layout::View;

struct QrCodeIterator<'a> {
    qr_code: &'a QrCode<'a>,
    points: Points,
}

impl<'a> QrCodeIterator<'a> {
    fn new(qr_code: &'a QrCode<'a>) -> Self {
        Self { qr_code, points: qr_code.bounding_box().points() }
    }
}

impl<'a> Iterator for QrCodeIterator<'a> {
    type Item = Pixel<BinaryColor>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.points.next()?;
        let is_on = self.qr_code.get_module(point.x, point.y);

        Some(Pixel(point, is_on.into()))
    }
}

impl<'a> Drawable for QrCode<'a> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.draw_iter(QrCodeIterator::new(self))
    }
}

struct StyledQrCodeIterator<'a> {
    inner: &'a StyledQrCode<'a>,
    points: Points,
}

impl<'a> StyledQrCodeIterator<'a> {
    // include pixeldoubling and border here??
    fn new(styled_qr_code: &'a StyledQrCode<'a>) -> Self {
        Self {
            inner: styled_qr_code,
            points: styled_qr_code.bounding_box().points(),
        }
    }
}

impl<'a> Iterator for StyledQrCodeIterator<'a> {
    type Item = Pixel<BinaryColor>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.points.next()?;
        Some(Pixel(point, self.inner.get_element_translated(point).into()))
    }
}

impl<'a> Drawable for StyledQrCode<'a> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.draw_iter(StyledQrCodeIterator::new(&self))
    }
}

pub struct StyledQrCode<'a> {
    pub qr_code: QrCode<'a>,
    pub position: Point,
    pub border_size_px: u32,
    pub scale: u32,
    pub is_inverted: bool,
}

impl<'a> StyledQrCode<'a> {
    pub fn inverted(mut self) -> Self {
        self.is_inverted = !self.is_inverted;
        self
    }

    pub fn with_scale(mut self, scale: u32) -> Self {
        self.scale = scale;
        self
    }

    pub fn reset(self) -> Self {
        Self::from(self.qr_code)
    }

    pub fn with_border(mut self, boder_size_px: u32) -> Self {
        self.border_size_px = boder_size_px;
        self
    }

    /// Get element using tranlates coordinates (i.e. where it is drawn on the screen)
    pub fn get_element_translated(&self, draw_point: Point) -> bool {
        self.get_element(self.local_point(draw_point))
    }

    /// Get element using local coordinates
    pub fn get_element(&self, point: Point) -> bool {
        let border_offset = Point::new(self.border_size_px as i32, self.border_size_px as i32);

        if self.is_border(point) {
            self.is_inverted
        } else {
            let scaled = (point - border_offset) / self.scale as i32;
            self.qr_code.get_module(scaled.x, scaled.y) ^ self.is_inverted
        }
    }

    pub fn body_size(&self) -> u32 {
        self.qr_code.size() as u32 * self.scale
    }

    pub fn is_border(&self, point: Point) -> bool {
        let (x, y) = point.into();
        let min = self.border_size_px as i32;
        let max = (self.border_size_px + self.body_size()) as i32;

        (x < min) || (x > max) || y < min || y > max
    }

    /// de-translate point with respect to drawing position, referenced to Point::zero()
    fn local_point(&self, draw_point: Point) -> Point {
        draw_point - self.position
    }
}

impl<'a> Dimensions for QrCode<'a> {
    fn bounding_box(&self) -> Rectangle {
        let qr_size = self.size() as u32;

        Rectangle {
            top_left: Point::zero(),
            size: Size { width: qr_size, height: qr_size },
        }
    }
}

impl<'a> Dimensions for StyledQrCode<'a> {
    fn bounding_box(&self) -> Rectangle {
        let total_size = self.body_size() + (self.border_size_px * 2);

        Rectangle {
            top_left: self.position,
            size: Size::new(total_size, total_size),
        }
    }
}

impl<'a> From<QrCode<'a>> for StyledQrCode<'a> {
    fn from(qr_code: QrCode<'a>) -> Self {
        Self {
            qr_code,
            position: Point::zero(),
            scale: 1,
            border_size_px: 2,
            is_inverted: false,
        }
    }
}

impl<'a> View for StyledQrCode<'a> {
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
    }

    fn bounds(&self) -> Rectangle {
        self.bounding_box()
    }
}
