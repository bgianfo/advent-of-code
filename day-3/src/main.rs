extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;


use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod fabric;

// Parse a line of input to it's descriptor.
fn parse_line(line : &String) -> fabric::Descriptor {

    let dim: Vec<_> = line.split("@").collect();
    let nums: Vec<_> = dim[1].split(":").collect();

    let location: Vec<_> = nums[0].split(",").collect();

    let x = location[0].trim();
    let y = location[1].trim();

    let sizes: Vec<_> = nums[1].split("x").collect();

    let w = sizes[0].trim();
    let h = sizes[1].trim();

    fabric::Descriptor {
        x: x.parse::<u32>().unwrap(),
        y: y.parse::<u32>().unwrap(),
        h: h.parse::<u32>().unwrap(),
        w: w.parse::<u32>().unwrap(),
    }
}

#[test]
fn test_parse_line() {
    let res = parse_line("#1218 @ 152,658: 11x17".to_string());
    assert_eq!(res.x, 152);
    assert_eq!(res.y, 658);
    assert_eq!(res.w, 11);
    assert_eq!(res.h, 17);
}

fn parse_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut fabric = fabric::FabricPiece::new();

    for line in reader.lines() {
        let lun = line.unwrap();
        let desc = parse_line(&lun);
        fabric.populate(&desc);
    }

    println!("Double booked count: {}", fabric.double_booked_count());

    let file2 = File::open(filename)?;
    let reader2 = BufReader::new(file2);
    for line in reader2.lines() {
        let lun = line.unwrap();
        let desc = parse_line(&lun);
        if fabric.is_uncontended(&desc) {
            println!("Unique: {}", lun);
        }
    }
    Ok(())
}

fn build_ui(application: &gtk::Application) -> std::io::Result<()>  {
    drawable(application, 2025, 2025, |_, cr| {

        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        println!("File: {}", filename);


        let mut fab = fabric::FabricPiece::new();

        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);


            for line in reader.lines() {
                let lun = line.unwrap();
                let desc = parse_line(&lun);

                // println!("Desc: {:?}", desc);

                fab.populate(&desc);
            }

            println!("Double booked count: {}", fab.double_booked_count());

            if let Ok(file2) = File::open(filename) {
                let reader2 = BufReader::new(file2);
                for line in reader2.lines() {
                    let lun = line.unwrap();
                    let desc = parse_line(&lun);
                    if fab.is_uncontended(&desc) {
                        println!("Unique: {}", lun);
                    }
                }
            }
        }

        // Start drawing
        cr.scale(2f64, 2f64);

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, 1000.0, 1000.0);
        cr.fill();

        for i in 0..1000 {
            for j in 0..1000  {
                let value = fab.get(i,j);
                
                if value != 0 {
                    let color = (value as f64 / 10.0) * 2.0;
                    cr.set_source_rgb(color, color, color);
                    cr.rectangle(i as f64, j as f64 , 0.25, 0.25);
                    cr.fill();
                }
            }
        }

        Inhibit(false)
    });

    Ok(())
}

fn main() {
    let application = gtk::Application::new("com.github.cairotest",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");
    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where F: Fn(&DrawingArea, &Context) -> Inhibit + 'static {
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });
    window.add(&drawing_area);
    window.show_all();
}
