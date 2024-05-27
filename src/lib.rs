// fc(z) = z^2 + c
// starts with z = 0;
// fc(0) = z0 = 0
// z1 = fc(fc(0)) = fc(0) = c

// bound: |c| <= 2; |fc(z)| <= 2
use plotters::prelude::*;

#[derive(Debug)]
enum Color {
    BLACK,
    YELLOW,
    GREEN,
    RED,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::BLACK => "rgb(0,0,0)",
            Color::YELLOW => "rgb(255, 255, 0)",
            Color::GREEN => "rgb(0, 255, 0)",
            Color::RED => "rgb(255, 0, 0)",
        }
    }

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
const OUT_FILE_NAME: &str = "./mandelbrot.png";

fn get_color_based_on_number_of_iterations(iterations: u8) -> Color {
    match iterations {
        0..=20 => Color::BLACK,
        21..=80 => Color::YELLOW,
        81..=160 => Color::GREEN,
        161..=u8::MAX => Color::RED,
    }
}

/// z = (x, yi)
/// |z|^2 = |x|^2 + |y|^2
/// |z| = sqrt(|x|^2 + |y|^2)
fn get_absolute_value_of_complex_number(complex: (f64, f64)) -> f64 {
    let adjacent_cathet = complex.0.powi(2);
    let opposite_cathet = complex.1.powi(2);
    let hypotenuse_squared = adjacent_cathet + opposite_cathet;
    hypotenuse_squared.sqrt()
}

/// z^2 = (x^2-y^2, 2xy)
fn calculate_square_of_complex_number(complex: (f64, f64)) -> (f64, f64) {
    let x = complex.0;
    let y = complex.1;

    (x.powi(2) - y.powi(2), 2.0 * x * y)
}

fn iterate(c: (f64, f64)) -> Color {
    let mut iterations: u8 = 0;
    if get_absolute_value_of_complex_number(c) > 2.0 {
        return get_color_based_on_number_of_iterations(iterations);
    }

    let mut z = (0.0, 0.0);

    while get_absolute_value_of_complex_number(z) <= 2.0 {
        iterations += 1;
        let z_squared = calculate_square_of_complex_number(z);
        let zn = (z_squared.0 + c.0, z_squared.1 + c.1);
        z = zn;

        if iterations >= MAX_ITERATIONS {
            break;
        }
    }

    get_color_based_on_number_of_iterations(iterations)
}

fn plot() {
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)
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
        let c = (
            xr.start + step.0 * (k % pw) as f64,
            yr.start + step.1 * (k / pw) as f64,
        );
        let color = iterate(c);

        plotting_area
            .draw_pixel(c, &color.as_plotters_color())
            .unwrap();
    });

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        plot();
    }
}
