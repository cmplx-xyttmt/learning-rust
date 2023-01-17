use num::complex::Complex;


// Converts between the output space (a grid of rows and columns) and a range that surrounds
// the Mandelbrot set (a continuous region near (0, 0))
fn calculate_mandelbrot(
    max_iters: usize, // if a value hasn't escaped before reaching the max no of iterations, it's considered to be within the Mandelbrot set.
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64, // parameters that specify the space we're searching for to look for members of the set.
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width); // Creates a container to house the data from each row.

    for img_y in 0..height {  // iterates row by row allowing us to print the output line by line.
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {

            // Calculate the proportion of the space covered in our output and converts that to points within the search space.
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 { // checks the escape condition and calculates the distance from the origin (0, 0), an absolute value of a complex number
            return i;
        }
        z = z * z + c;  // repeatedly mutates z to check whether c lies within the Mandelbrot set.
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{line}");
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0,
                                          -1.0, 1.0, 100, 24);

    render_mandelbrot(mandelbrot);
}
