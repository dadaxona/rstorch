use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use ndarray::{Array3, s};

pub struct FrameEditor {
    font: Font<'static>,
}

impl FrameEditor {
    pub fn new() -> Self {
        let font_data = include_bytes!("fonts/arial.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Font xatosi");
        FrameEditor { font }
    }

    // Maksimal tezlik: To'g'ridan-to'g'ri piksellarni o'zgartiramiz
    pub fn draw_rect_inplace(&self, frame: &mut Array3<u8>, x: i32, y: i32, width: u32, height: u32, thickness: i32) {
        let (h, w, _) = frame.dim();
        let color = [255, 0, 0]; // Qizil

        for t in 0..thickness {
            let curr_x = (x - t) as usize;
            let curr_y = (y - t) as usize;
            let curr_w = (width + t as u32 * 2) as usize;
            let curr_h = (height + t as u32 * 2) as usize;

            // Chegaradan chiqib ketmasligini tekshiramiz
            if curr_x + curr_w < w && curr_y + curr_h < h {
                // Yuqori va pastki chiziqlar
                for i in curr_x..(curr_x + curr_w) {
                    frame.slice_mut(s![curr_y, i, ..]).assign(&ndarray::aview1(&color));
                    frame.slice_mut(s![curr_y + curr_h, i, ..]).assign(&ndarray::aview1(&color));
                }
                // Chap va o'ng chiziqlar
                for j in curr_y..(curr_y + curr_h) {
                    frame.slice_mut(s![j, curr_x, ..]).assign(&ndarray::aview1(&color));
                    frame.slice_mut(s![j, curr_x + curr_w, ..]).assign(&ndarray::aview1(&color));
                }
            }
        }
    }

    pub fn put_text_inplace(&self, frame: &mut Array3<u8>, text: &str, x: i32, y: i32) {
        let (h, w, _) = frame.dim();
        // Matn chizish uchun vaqtincha RgbImage view yaratamiz (bu ham xotira ko'chirmaydi)
        let mut img = RgbImage::from_raw(w as u32, h as u32, frame.as_slice_mut().unwrap().to_vec()).unwrap();
        
        let scale = Scale { x: 30.0, y: 30.0 };
        draw_text_mut(&mut img, Rgb([0, 255, 0]), x, y, scale, &self.font, text);

        // Faqat o'zgargan matn piksellarini qaytaramiz
        frame.as_slice_mut().unwrap().copy_from_slice(img.as_raw());
    }
}