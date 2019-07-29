extern crate image;

mod img;
use img::Img;
//use math::round;
//use math::abs;
//use math::sqrt;

/*
mod obj;
use obj::Obj;
*/

fn main() {
    // create a new image
    let w = 500;
    let h = 500;
    let mut img = Img::new(w, h);

    /* draw a nice wonderful beautiful circle
    */
    for _ in 1..10000000 {
        circle(&mut img, 200, 200, 30, (255, 255, 0));
    }

    /* draw a nice wonderful beautiful circle
    for _ in 1..10000000 {
        circle2(&mut img, 200., 200., 30., (255, 255, 0));
    }
    */

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

// Bresenham (unoptimized) circle
fn circle2(img: &mut Img, x0: f32, y0: f32, radius:  f32, color: (u8,u8,u8)) {

    let mut x_coord = radius;
    let mut y_coord = 0.;
    while x_coord >= y_coord {
        // this is just a silly way of looping over [-1,1]
        let mut i:f32 = -1.;
        while i <= 1. {
            let mut j:f32 = -1.;
            while j <= 1. {
                // place point
                let x1 = x_coord*i + x0;
                let y1 = y_coord*j + y0;
                img.put(x1 as u32, y1 as u32, color);

                // mirror point
                let x1 = y_coord*i + x0;
                let y1 = x_coord*j + y0;
                img.put(x1 as u32, y1 as u32, color);

                j += 2.;
            }

            i += 2.;
        }

        x_coord = (x_coord*x_coord - 2.*y_coord - 1.).abs().sqrt();
        y_coord += 1.;
    }
}

