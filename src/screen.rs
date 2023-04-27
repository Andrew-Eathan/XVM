use cursive::{views::{TextView, Canvas}, theme::{Theme, ColorStyle, Color}, view::Resizable, Printer};

use crate::processor::XCPU;

extern crate cursive;

pub struct XTerminal {
    pub cursive_term: cursive::CursiveRunnable,
    pub width: u16,
    pub height: u16,
    pub mem: Vec<Vec<u32>>
}

impl XTerminal {
	pub fn new() -> XTerminal {
		XTerminal {
			cursive_term: cursive::default(),
			width: 80,
			height: 24,
            mem: Vec::new()
		}
	}

    fn front_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
        // We return a full 24-bits RGB color, but some backends
        // will project it to a 256-colors palette.
        Color::Rgb(
            x * (255 / x_max),
            y * (255 / y_max),
            (x + 2 * y) * (255 / (x_max + 2 * y_max)),
        )
    }
    
    // Gradient for the background color
    fn back_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
        // Let's try to have a gradient in a different direction than the front color.
        Color::Rgb(
            128 + (2 * y_max + x - 2 * y) * (128 / (x_max + 2 * y_max)),
            255 - y * (255 / y_max),
            255 - x * (255 / x_max),
        )
    }

    pub fn draw(_: &(), p: &Printer) {
        let x_max = p.size.x as u8;
        let y_max = p.size.y as u8;

        for x in 0..x_max {
            for y in 0..y_max {
                let style = ColorStyle::new(
                    XTerminal::front_color(x, y, x_max, y_max),
                    XTerminal::back_color(x, y, x_max, y_max),
                );

                p.with_color(style, |printer| {
                    printer.print((x, y), "+");
                });
            }
        }

        p.print((x_max - 4, y_max - 2), x_max.to_string().as_str());
        p.print((x_max - 4, y_max - 1), y_max.to_string().as_str());
    }

    pub fn initialise(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.mem = vec![vec![0; width]; height];

        let termview = Canvas::new(()).with_draw(Self::draw).fixed_size((width, height));

        self.cursive_term = cursive::default();
        self.cursive_term.add_global_callback('q', |s| s.quit());
        self.cursive_term.add_layer(termview);
        self.cursive_term.run();
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) {
        
    }
}
