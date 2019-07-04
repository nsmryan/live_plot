use minifb::{Key, WindowOptions, Window};
use plotters::prelude::*;


const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut plot_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 4];

    let mut window = Window::new("Live Plot", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        plot(&mut plot_buffer, WIDTH as u32, HEIGHT as u32).unwrap();

        for (index, pixel) in buffer.iter_mut().enumerate() {
            let pixel_data = ((plot_buffer[index * 3 + 0] as u32) << 16) |
                             ((plot_buffer[index * 3 + 1] as u32) << 8)  |
                             ((plot_buffer[index * 3 + 2] as u32) << 0);

            *pixel = pixel_data;
        }

        window.update_with_buffer(&buffer).unwrap();
    }

}

fn plot(buffer: &mut Vec<u8>, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::with_buffer(buffer, (width, height)).into_drawing_area();

    root.fill(&White)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;
 
    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
        &Red))?
        .label("y = x^2")
        .legend(|(x,y)| Path::new(vec![(x,y), (x + 20,y)], &Red));

    chart.configure_series_labels()
        .background_style(&White.mix(0.8))
        .border_style(&Black)
        .draw()?;

    //root.present().unwrap();

    Ok(())
}
