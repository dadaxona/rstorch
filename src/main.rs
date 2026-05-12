#![allow(non_snake_case)]
mod rs_cv;
use std::time::Instant;
use rs_cv::rscv::Camera;
use rs_cv::editor::FrameEditor;
// create_window deb sinab ko'ring
use show_image::{create_window, ImageView, ImageInfo}; 

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let uri = "rtsp://admin:A112233a@192.168.1.213:554/Streaming/Channels/101";
    let uri = "Test_video.mp4";
    // let uri = "img.jpg";

    let cap = Camera::new(&uri);
    let editor = FrameEditor::new();
    let window = create_window("Monitoring", Default::default())?;

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_text = String::from("FPS: 0");

    loop {
        if let Some(mut frame) = cap.read() {
            frame_count += 1;
            if last_time.elapsed().as_secs() >= 1 {
                let fps = frame_count as f64 / last_time.elapsed().as_secs_f64();
                fps_text = format!("FPS: {:.2}", fps);
                frame_count = 0;
                last_time = Instant::now();
            }
            // println!("{:?}", frame);
            editor.draw_rect_inplace(&mut frame, 200, 150, 100, 100, 3);
            editor.put_text_inplace(&mut frame, &fps_text, 10, 10);
            editor.put_text_inplace(&mut frame, "SYSTEM: ULTRA FAST", 10, 45);
            let (h, w, _) = frame.dim();
            let image_view = ImageView::new(
                ImageInfo::rgb8(w as u32, h as u32),
                frame.as_slice().unwrap()
            );
            window.set_image("live", image_view)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}