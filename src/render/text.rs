// use embedded_graphics::draw_target::DrawTarget;
// use embedded_graphics::{Drawable, Pixel};
// use embedded_graphics::geometry::Dimensions;
// use embedded_graphics::mono_font::MonoTextStyle;
// use embedded_graphics::pixelcolor::BinaryColor;
// use embedded_graphics::prelude::{Point, Size};
// use embedded_graphics::primitives::arc::Points;
// use embedded_graphics::primitives::Rectangle;
// use embedded_graphics::text::Text;
// 
// struct StyledTextIterator {
//     inner: StyledText,
//     points: Points,
// }
// 
// impl StyledTextIterator {
//     fn new(styled_text: StyledText) -> Self {
//         Self {
//             inner: styled_text,
//             points: styled_text.bounding_box().points(),
//         }
//     }
// }
// 
// impl Iterator for StyledTextIterator {
//     type Item = embedded_graphics::Pixel<BinaryColor>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         let point = self.points.next()?;
//         Some(Pixel(point, self.inner.get_element_translated(point).into()))
//     }
// }
// 
// 
// struct StyledText {
//     pub text: Points,
//     pub position: Point,
//     pub width: u32,
//     pub height: u32,
//     pub border_size_px: u32,
// }
// 
// impl Drawable for StyledText {
//     type Color = BinaryColor;
//     type Output = ();
// 
//     fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
//     where
//         D: DrawTarget<Color=Self::Color>,
//     {
//         target.draw_iter(StyledTextIterator::new(&self))
//     }
// }
// 
// impl Dimensions for StyledText {
//     fn bounding_box(&self) -> Rectangle {
//         let total_width = self.width + 2 * self.border_size_px;
//         let total_height = self.height + 2 * self.border_size_px;
// 
//         Rectangle {
//             top_left: self.position,
//             size: Size::new(total_width, total_height),
//         }
//     }
// 
// 
