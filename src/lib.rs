#![allow(dead_code)]

use plotters::prelude::*;

#[derive(Clone, Copy)]
struct Complex(f64, f64);

impl Complex {
    /// z = (x, yi)
    /// |z|^2 = |x|^2 + |y|^2
    /// |z| = sqrt(|x|^2 + |y|^2)
    fn get_absolute_value_of_complex_number(complex: Complex) -> f64 {
        let adjacent_cathet = complex.0.powi(2);
        let opposite_cathet = complex.1.powi(2);
        let hypotenuse_squared = adjacent_cathet + opposite_cathet;
        hypotenuse_squared.sqrt()
    }

    /// z^2 = (x^2-y^2, 2xy)
    fn calculate_square_of_complex_number(complex: Complex) -> Complex {
        let x = complex.0;
        let y = complex.1;

        Complex(x.powi(2) - y.powi(2), 2.0 * x * y)
    }
}

impl From<Complex> for (f64, f64) {
    fn from(val: Complex) -> Self {
        (val.0, val.1)
    }
}

#[allow(clippy::upper_case_acronyms)]
enum Set {
    MANDELBROT,
    JULIA,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Color {
    BLACK,
    YELLOW,
    GREEN,
    RED,
}

impl Color {
    fn as_plotters_color(&self) -> RGBColor {
        match self {
            Color::BLACK => RGBColor(0, 0, 0),
            Color::YELLOW => RGBColor(255, 255, 0),
            Color::GREEN => RGBColor(0, 255, 0),
            Color::RED => RGBColor(255, 0, 0),
        }
    }
}

const MAX_ITERATIONS: u8 = u8::MAX;
const MANDELBROT_OUT_FILE_NAME: &str = "./mandelbrot.png";
const JULIA_OUT_FILE_NAME: &str = "./julia.png";

fn get_color_based_on_number_of_iterations(iterations: u8) -> Color {
    match iterations {
        0..=20 => Color::RED,
        21..=80 => Color::YELLOW,
        81..=160 => Color::GREEN,
        161..=u8::MAX => Color::BLACK,
    }
}

/// Julia sets differ from Mandelbrot in they change the `z` variable
/// and not the `c`.
///
fn julia(mut z: Complex) -> Color {
    let mut iterations: u8 = 0;

    if Complex::get_absolute_value_of_complex_number(z) > 2.0 {
        return get_color_based_on_number_of_iterations(iterations);
    }

    // Random value to c
    let c = Complex(-0.79, 0.15);

    // fc(z) = z^2 + c
    while Complex::get_absolute_value_of_complex_number(z) <= 2.0 {
        iterations += 1;
        let z_squared = Complex::calculate_square_of_complex_number(z);
        let zn = Complex(z_squared.0 + c.0, z_squared.1 + c.1);
        z = zn;

        if iterations == MAX_ITERATIONS {
            break;
        }
    }

    get_color_based_on_number_of_iterations(iterations)
}

fn mandelbrot(c: Complex) -> Color {
    let mut iterations: u8 = 0;

    // Mandelbrot set is bounded by |fc(z)| <= 2, which is the same as bounding |c| <= 2
    if Complex::get_absolute_value_of_complex_number(c) > 2.0 {
        return get_color_based_on_number_of_iterations(iterations);
    }

    // initial z = 0
    let mut z = Complex(0.0, 0.0);

    // fc(z) = z^2 + c
    while Complex::get_absolute_value_of_complex_number(z) <= 2.0 {
        iterations += 1;
        let z_squared = Complex::calculate_square_of_complex_number(z);
        let zn = Complex(z_squared.0 + c.0, z_squared.1 + c.1);
        z = zn;

        if iterations == MAX_ITERATIONS {
            break;
        }
    }

    get_color_based_on_number_of_iterations(iterations)
}

fn plot(set: Set) {
    let path = match set {
        Set::MANDELBROT => MANDELBROT_OUT_FILE_NAME,
        Set::JULIA => JULIA_OUT_FILE_NAME,
    };
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let cartesian_2d = match set {
      Set::MANDELBROT => (-2.1f64..0.6f64, -1.2f64..1.2f64),
      Set::JULIA => (-2.1f64..2.1f64, -1.2f64..1.2f64),
  };

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(cartesian_2d.0, cartesian_2d.1)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (xr, yr) = (chart.x_range(), chart.y_range());

    let step = (
        (xr.end - xr.start) / pw as f64,
        (yr.end - yr.start) / ph as f64,
    );
    (0..(pw * ph)).for_each(|k| {
        let c = Complex(
            xr.start + step.0 * (k % pw) as f64, // this goes back to zero once we reach the end of the width of the graph
            yr.start + step.1 * (k / pw) as f64, // this breaks the line (goes to the next row) once it reaches the end of the width of the graph
        );
        let color = match set {
            Set::MANDELBROT => mandelbrot(c),
            Set::JULIA => julia(c),
        };

        plotting_area
            .draw_pixel(c.into(), &color.as_plotters_color())
            .expect("Could not plot the graph. Something went wrong.");
    });

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        plot(Set::MANDELBROT);
        plot(Set::JULIA);
    }
}
