extern crate image;

mod img;
use img::Img;
use std::time::Instant;

fn main() {
    // create a new image
    let w = 500;
    let h = 500;
    let mut img = Img::new(w, h);

    // draw a nice wonderful beautiful circle
    let now = Instant::now();
    for _ in 1..100000 {
        circle(&mut img, 200, 200, 100, (255, 255, 0));
    }
    println!("{:?}", Instant::now().duration_since(now));

    // draw a nice wonderful beautiful circle
    let now = Instant::now();
    for _ in 1..100000 {
        circle2(&mut img, 200., 200., 100., (255, 255, 0));
    }
    println!("{:?}", Instant::now().duration_since(now));

    // save the image
    img.save();

    println!("image saved to <images/output.jpg>")
}

/**
 * Bresenham circle (with cool optimizations)
 * mostly stolen from here:
 * http://weber.itn.liu.se/~stegu/circle/circlealgorithm.pdf
 */
fn circle(img: &mut Img, x0: i32, y0: i32, radius:  i32, color: (u8,u8,u8)) {

    let mut x_coord = 0;
    let mut y_coord = radius;
    let mut d_value = 5 - (radius << 2); // bit-shifts = awesomeness
    let mut da0     = 12;
    let mut db0     = 20 - (radius << 3); // more bit-shifts = more awesomeness

    while x_coord <= y_coord {
        img.put((x0 + x_coord) as u32, (y0 + y_coord) as u32, color);
        img.put((x0 - x_coord) as u32, (y0 + y_coord) as u32, color);
        img.put((x0 + x_coord) as u32, (y0 - y_coord) as u32, color);
        img.put((x0 - x_coord) as u32, (y0 - y_coord) as u32, color);

        img.put((y0 + y_coord) as u32, (x0 + x_coord) as u32, color);
        img.put((y0 - y_coord) as u32, (x0 + x_coord) as u32, color);
        img.put((y0 + y_coord) as u32, (x0 - x_coord) as u32, color);
        img.put((y0 - y_coord) as u32, (x0 - x_coord) as u32, color);

        if d_value < 0 {
            d_value = d_value + da0;
            db0 = db0 + 8;
        } else {
            y_coord = y_coord - 1;
            d_value = d_value + db0;
            db0 = db0 + 16;
        }

        x_coord = x_coord + 1;
        da0 = da0 + 8;
    }
}

/**
 * Bresenham (unoptimized) circle
 * written by following the Wikipedia page:
 * https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
 */
fn circle2(img: &mut Img, x0: f32, y0: f32, radius:  f32, color: (u8,u8,u8)) {

    let mut x_coord = radius;
    let mut y_coord = 0.;
    while x_coord >= y_coord {
        img.put((x0 + x_coord) as u32, (y0 + y_coord) as u32, color);
        img.put((x0 - x_coord) as u32, (y0 + y_coord) as u32, color);
        img.put((x0 + x_coord) as u32, (y0 - y_coord) as u32, color);
        img.put((x0 - x_coord) as u32, (y0 - y_coord) as u32, color);

        img.put((y0 + y_coord) as u32, (x0 + x_coord) as u32, color);
        img.put((y0 - y_coord) as u32, (x0 + x_coord) as u32, color);
        img.put((y0 + y_coord) as u32, (x0 - x_coord) as u32, color);
        img.put((y0 - y_coord) as u32, (x0 - x_coord) as u32, color);

        x_coord = (x_coord*x_coord - 2.*y_coord - 1.).abs().sqrt();
        y_coord += 1.;
    }
}

