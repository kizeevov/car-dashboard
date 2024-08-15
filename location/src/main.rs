use plotters::backend::{BitMapBackend, DrawingBackend};
use plotters::drawing::IntoDrawingArea;
use plotters::style::{RGBColor, GREEN, WHITE};
use slint::SharedPixelBuffer;
slint::include_modules!();

fn render_img() -> slint::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(640, 480);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    // Plotters requires TrueType fonts from the file system to draw axis text - we skip that for
    // WASM for now.
    #[cfg(target_arch = "wasm32")]
    let backend = wasm_backend::BackendWithoutText { backend };

    backend
        .draw_circle((10, 10), 7, &RGBColor(255, 1, 34), true)
        .unwrap();

    let source_image = {
        let mut cat_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cat_path.push("assets/5-10-10.png");
        println!("{:?}", cat_path);

        image::open(&cat_path)
            .expect("Error loading cat image")
            .into_bytes()
    };

    let source_image_2 = {
        let mut cat_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cat_path.push("assets/5-11-10.png");
        println!("{:?}", cat_path);

        image::open(&cat_path)
            .expect("Error loading cat image")
            .into_bytes()
    };

    backend
        .blit_bitmap((-150, 0), (256, 256), &source_image)
        .unwrap();
    backend
        .blit_bitmap((256 - 150, 0), (256, 256), &source_image_2)
        .unwrap();

    // let root = backend.into_drawing_area();

    // root.draw()

    // root.fill(&GREEN).expect("error filling drawing area");

    // drop(root);
    drop(backend);

    slint::Image::from_rgb8(pixel_buffer)
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let map_control = map_ui::slint::MapControl::new();
    ui.on_render_img(move || map_control.render());

    // var mapControl = new Mapsui.UI.Avalonia.MapControl();
    // mapControl.Map?.Layers.Add(Mapsui.Tiling.OpenStreetMap.CreateTileLayer());

    // let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    // let map = DeclarativeGeoMap::new();
    // ui.on_render_img(render_img);
    // ui.on_pan_map(move |x, y| {
    //     map.pan(x, y);
    // });

    ui.run()
}
