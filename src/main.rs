use std::time::Duration;
use embedded_graphics::prelude::Dimensions;
use ql_raster::{
    commands::{Commands, PrintInfo, VariousMode},
    prelude::*,
    printer::PTouchPrinter,
};
use ql_raster::prelude::display::Display;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
};
use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use embedded_graphics::text::Text;
use ql_raster::render::{Render, RenderConfig};

fn main() {
    println!("Hello, world!");
    // let mut printer =
    //     printer::from_addr("labelprinter_1:9100").expect("Unable to connect to printer!");
    //
    // let status = printer.get_snmp_status();
    // println!("Status {:?}", status);
    // let name = printer.get_snmp_name();
    // println!("name {:?}", name);
    // let model = printer.get_snmp_model();
    // println!("model {:?}", model);
    //
    // let status = printer.get_snmp_status();
    // println!("Status {:?}", status);



    let mut render = Render::new(RenderConfig::default());


    render.render_text("Hello Rust!", Point::new(1,61)).expect("Failed to render text");
    render.render_qrcode("https://lib.rs/crates/embedded-graphics-transform",Point::new(1,0)).expect("TODO: panic message");
    render.show().expect("cannot show");

    // sleep 1 sec
    std::thread::sleep(Duration::from_secs(1));    
    // printer.print_data(raster_lines).unwrap()
}


