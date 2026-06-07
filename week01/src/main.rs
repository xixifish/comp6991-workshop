use std::{io::{self, ErrorKind}};
use bmp::{Image, Pixel, consts};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    // Check the bmp file
    match bmp::open(&path) {
        Ok(_) => {}, // If the file exists and valid, do nothing and continue
        Err(e) => match &e.kind { // If capture an error of the file
            // Missing is one error belongs to BmpIoError
            bmp::BmpErrorKind::BmpIoError(io_err) => match io_err.kind() {
                ErrorKind::NotFound => {}, // If not found, do nothing and continue
                _ => { eprintln!("Error: {:?}", e); return; },
            },
            _ => { eprintln!("Error: {:?}", e); return; },
            // Only ends the program when the file is invalid, and print out the error
        }
    }

    // Operate the bmp file
    match operation.as_str() {
        "pixel" => draw_pixel(path.as_str()),
        "diagonal-line" => draw_diagonal_line(path.as_str(), 100, 100),
        "x" => draw_x(path.as_str()),
        "house" => draw_house(path.as_str()),
        "rectangle" => {
            match read_numbers() {
                Ok((width, height, x, y)) => draw_rectangle(path.as_str(), width, height, x, y),
                Err(e) => eprintln!("Error: {e}"),
            }
        },
        "filled-squared" => draw_filled_square(path.as_str()),
        "rainbow-flag" => draw_rainbow_flag(path.as_str()),
        "finland-flag" => draw_finland_flag(path.as_str()),
        "iceland-flag" => draw_iceland_flag(path.as_str()),
        "aboriginal-flag" => draw_aboriginal_flag(path.as_str()),
        "sine" => draw_sine(path.as_str()),
        _ => eprintln!("The operation {operation} was not recognised!"),

    };

}


/// Helper functions 
fn create_image(width:u32, height: u32) -> Image {
    let image = Image::new(width, height);
    image
}

// Helper function to read and check if the width, height, and coordinates (x, y) are legal
fn read_numbers() -> Result<(u32, u32, u32, u32), String> {
    // Read width and height of the rectangle, and the coordinates of the shape
    println!("The canvas is 200 x 200.");

    println!("What is the width of the rectangle?");
    let width = read_one_number()?;

    println!("What is the height of the rectangle? If it's same as width, then you will get a square.");
    let height: u32 = read_one_number()?;

    println!("What is the coordinates of the shape? Enter as x, y");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;

    let coordinates: Vec<&str> = input.split(',').collect();

    // Check if the input has two numbers
    if coordinates.len() != 2 {
        return Err(format!(
            "Expected two values separated by a comma, but got {}.",
            coordinates.len()
        ));
    }

    let x = coordinates[0]
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("'{}' is not a valid number.", coordinates[0].trim()))?;
    let y = coordinates[1]
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("'{}' is not a valid number.", coordinates[1].trim()))?;

    Ok((width, height, x, y))

}

fn read_one_number() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input {e}"))?;
    input
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("'{}' is not a valid number.", input.trim()))
}

/// Drawing functions
fn draw_pixel(path: &str) {
    let mut image = create_image(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn draw_diagonal_line(path: &str, width: u32, height: u32) {
    if width != height {
        println!("Error: only can draw diagnal line in a square image.")
    } else {
        let mut image = create_image(width, height);

        // Draw diagonal line
        for y in 0..height {
            for x in 0..width {
                if x == y {
                    image.set_pixel(x, y, consts::BLUE);
                } else {
                    image.set_pixel(x, y, consts::WHITE);
                }
            }
        }

        image.save(path).expect("This should save correctly.");
        
    }
}

fn draw_x(path: &str) {

    let mut image = create_image(100, 100);

    // Draw x across the whole canvas
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            if x == y {
                image.set_pixel(x, y, consts::PURPLE);
            } else if x + y == image.get_width() - 1 {
                image.set_pixel(x, y, consts::PURPLE);
            } else {
                image.set_pixel(x, y, consts::WHITE);
            }
        }
    }

    image.save(path).expect("This should save correctly.");
}

fn draw_house(path: &str) {

    let mut image = create_image(200, 200);

    // Fill the background with white
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            image.set_pixel(x, y, consts::WHITE);
        }
    }

    // Size of the house and margin space
    let body_height: u32 = 100;
    let body_width: u32 = 160; // roof width is same as body width
    let roof_height: u32 = 60;
    let space: u32 = 20; // margin space (top, right, bottom, left)

    // 1. Draw house body
    for y in (space + roof_height - 1)..(space + roof_height + body_height) {
        for x in (space - 1)..(space + body_width) {
            // Draw horizontal lines
            if (y == space + roof_height - 1) || (y == space + roof_height + body_height - 1) {
                image.set_pixel(x, y, consts::BLACK);
            //Draw vertical lines
            } else if (x == space - 1) || (x == space + body_width - 1) {
                image.set_pixel(x, y, consts::BLACK);
            }
        }
    }

    // 2. Draw roof
    let top_y = space - 1;
    let bottom_y = space + roof_height - 1;
    let apex_x = space - 1 + body_width / 2;
    // let left_x = space - 1;
    // let right_x = space + body_width - 1;

    let v_span = bottom_y - top_y;
    let h_span = body_width / 2;

    for y in top_y..bottom_y {
        let down = y - top_y;
        let offset = down * h_span / v_span;

        let lx = apex_x - offset;
        let rx = apex_x + offset;

        image.set_pixel(lx, y, consts::RED);
        image.set_pixel(rx, y, consts::RED);
    }

    image.save(path).expect("This should save correctly.");

}

fn draw_rectangle(path: &str, width: u32, height: u32, x: u32, y: u32) {

    let mut image = create_image(200, 200);

    for pixel_y in y..(y + height) {
        for pixel_x in x..(x + width) {
            if pixel_y == y || pixel_y == y + height - 1 {
                image.set_pixel(pixel_x, pixel_y, consts::WHITE);
            } else if pixel_x == x || pixel_x == x + width - 1 {
                image.set_pixel(pixel_x, pixel_y, consts::WHITE);
            }
        }
    }

    image.save(path).expect("This should save correctly.");
    println!("Drawing is done. ");
}

fn draw_filled_square(path: &str) {

    let mut image = create_image(200, 200);

    // Start coordinates
    let x_start: u32 = 20;
    let y_start: u32 = 20;

    // Square width
    let side: u32 = 100;

    for y in y_start..(y_start + side) {
        for x in x_start..(x_start + side) {
            if x == x_start || x == x_start + side - 1 {
                image.set_pixel(x, y, consts::WHITE);
            } else if y == y_start || y == y_start + side - 1 {
                image.set_pixel(x, y, consts::WHITE);
            } else {
                image.set_pixel(x, y, consts::BLUE);
            }
        }
    }
    image.save(path).expect("This should save correctly.");

}

fn draw_rainbow_flag(path: &str) {

    let mut image = create_image(600, 1000);

    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            if y <= 199 {
                image.set_pixel(x, y,consts::RED);
            } else if y >= 200 && y <= 399 {
                image.set_pixel(x, y, consts::ORANGE);
            } else if y >= 400 && y <= 599 {
                image.set_pixel(x, y, consts::YELLOW);
            } else if y >= 600 && y <= 799 {
                image.set_pixel(x, y, consts::GREEN);
            } else{
                image.set_pixel(x, y, consts::BLUE);
            }
        }
    }
    image.save(path).expect("This should save correctly.");

}

fn draw_finland_flag(path: &str) {

    let mut image = create_image(180, 110);

    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            if y >= 40 && y <= 60 {
                image.set_pixel(x, y, consts::BLUE);
            } else if x >= 50 && x <= 70 {
                image.set_pixel(x, y, consts::BLUE);
            } else {
                image.set_pixel(x, y, consts::WHITE);
            }
        }
    }

    image.save(path).expect("This should save correctly.");
}

fn draw_iceland_flag(path: &str) {

    let width: u32 = 200;
    let height: u32 = 100;
    let mut image = create_image(width, height);

    // Cross geometry. The vertical arm sits left of centre (towards the hoist)
    let cross_x: u32 = 60;           // centre column of the cross
    let cross_y: u32 = height / 2;   // centre row of the cross
    let white_arm: u32 = 9;          // half-thickness of the white cross arm
    let red_arm: u32 = 4;            // half-thickness of the red cross arm

    for y in 0..height {
        for x in 0..width {
            let in_white = within(x, cross_x, white_arm) || within(y, cross_y, white_arm);
            let in_red = within(x, cross_x, red_arm) || within(y, cross_y, red_arm);

            let colour = if in_red {
                consts::RED
            } else if in_white {
                consts::WHITE
            } else {
                consts::BLUE
            };

            image.set_pixel(x, y, colour);
        }
    }

    image.save(path).expect("This should save correctly.");

}

// Helper function for draw_iceland_flag
fn within(value: u32, centre: u32, half_thickness: u32) -> bool {
    let low = centre.saturating_sub(half_thickness);
    let high = centre + half_thickness;
    value >= low && value < high
}

fn draw_aboriginal_flag(path: &str) {
    let width = 200;
    let height = 100;
    let mut image = create_image(width, height);

    // Yellow circle
    let circle_centre_x = width / 2;
    let circle_centre_y = height / 2;
    let radius: u32 = 30;

    for y in 0..height {
        for x in 0..width {
            let dx = x as i32 - circle_centre_x as i32;
            let dy = y as i32 - circle_centre_y as i32;

            let in_circle = dx * dx + dy * dy <= (radius * radius) as i32;

            let colour = if in_circle {
                consts::YELLOW
            } else if y < height / 2 {
                consts::BLACK
            } else {
                consts::RED
            };

            image.set_pixel(x, y, colour);
        }
    }

    image.save(path).expect("This should save correctly.");
}

fn draw_sine(path: &str) {
    let width: u32 = 200;
    let height: u32 = 150;
    let mut image = create_image(width, height);

    let cycles = 3.0;  // How many full waves across the image
    let centre = height as f64 / 2.0;  
    let amplitude = 10.0;  // 20 pixels tall total

    for x in 0..width {
        let angle = (x as f64 / width as f64) * cycles * std::f64::consts::PI * 2.0;
        let value = angle.sin();  // between -1.0 and 1.0
        let y_float = centre - value * amplitude;
        let y = y_float as u32;

        image.set_pixel(x, y, consts::WHITE);
    }

    image.save(path).expect("This could save correctly.");

}








// Check the bmp file
// If the file is invalid, return the error
// Otherwise do nothing
// fn check_bmp(path: &str) -> Result<(), bmp::BmpError> {
//     match bmp::open(path) {
//         Ok(_) => Ok(()),
//         Err(e) => match e.kind {
//             bmp::BmpErrorKind::BmpIoError(io_err)
//                 if io_err.kind() == ErrorKind::NotFound => Ok(()), // matching arm with a if statement: match guard
//             _ => Err(e),
//         },
//     }
// }
