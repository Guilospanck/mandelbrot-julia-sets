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

#[derive(Debug)]
struct CoordinatesAndColors {
    coordinates: Vec<(f64, f64)>,
    color: Color,
}

impl CoordinatesAndColors {
    fn new() -> Self {
        Self {
            coordinates: vec![],
            color: Color::BLACK,
        }
    }
}

fn get_color_based_on_number_of_iterations(iterations: u8) -> Color {
    match iterations {
        0..=20 => Color::BLACK,
        21..=80 => Color::YELLOW,
        81..=160 => Color::GREEN,
        161..=u8::MAX => Color::RED,
    }
}

fn iterate(c: f64) -> CoordinatesAndColors {
    if c < -2.0 || c > 2.0 {
        return CoordinatesAndColors::new();
    }

    let mut coordinates: Vec<(f64, f64)> = vec![];

    let mut z = c;
    coordinates.push((0.0, 0.0)); // z0
    coordinates.push((0.0, c)); // z1

    let mut iterations: u8 = 0;

    while z.abs() <= 2.0 {
        iterations += 1;
        let zn = z.powi(2) + c;
        coordinates.push((z, zn));
        z = zn;

        if iterations >= MAX_ITERATIONS {
            break;
        }
    }

    let color = get_color_based_on_number_of_iterations(iterations);

    CoordinatesAndColors { coordinates, color }
}

fn plot(mandelbrot_set: Vec<CoordinatesAndColors>) {
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

    for CoordinatesAndColors { coordinates, color } in mandelbrot_set {
        for coordinate in coordinates {
            plotting_area
                .draw_pixel(coordinate, &color.as_plotters_color())
                .unwrap();
        }
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut results = vec![];

        let mut c: f64 = -2.0;
        while c.abs() <= 2.0 {
            let result = iterate(c);
            results.push(result);
            c += 0.01
        }

        plot(results);
    }
}
