#![allow(non_snake_case)]

use gstreamer as gst;
use gstreamer_app as gst_app;
use gst::prelude::*;
use ndarray::Array3;
use std::sync::mpsc::{channel, Receiver};
use std::env;
use std::path::Path;

pub struct Camera {
    receiver: Receiver<Array3<u8>>,
}

impl Camera {
    
    pub fn new(uri: &str) -> Self {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let base_path = Path::new(&manifest_dir).join("src").join("rs_cv").join("msvc_x86_64");
        
        // DLL yo'llarini sozlash
        unsafe {
            let bin_path = base_path.join("bin");
            let plugin_path = base_path.join("lib").join("gstreamer-1.0");
            
            let path_var = env::var_os("PATH").unwrap_or_default();
            let mut paths = env::split_paths(&path_var).collect::<Vec<_>>();
            paths.insert(0, bin_path);
            env::set_var("PATH", env::join_paths(paths).unwrap());
            env::set_var("GST_PLUGIN_PATH", plugin_path.to_str().unwrap().replace("\\\\?\\", ""));
            env::set_var("GST_REGISTRY_REUSE_PLUGIN_SCANNER", "no");
        }

        gst::init().expect("GStreamer init xatosi");
        let (tx, rx) = channel();

        // YO'LNI FORMATLASH:
        let pipeline_str = if uri.starts_with("rtsp://") {
            // RTSP holati
            format!(
                "rtspsrc location=\"{}\" latency=100 protocols=tcp ! decodebin ! videoconvert ! video/x-raw,format=RGB ! appsink name=sink",
                uri
            )
        } else {
            // Mahalliy fayl holati (src ichidan qidiradi)
            let video_path = Path::new(&manifest_dir).join("src").join(uri);
            if !video_path.exists() {
                panic!("Fayl topilmadi: {:?}", video_path);
            }
            let full_path = video_path.canonicalize().unwrap()
                .to_string_lossy()
                .replace("\\\\?\\", "")
                .replace("\\", "/");
            
            format!(
                "filesrc location=\"{}\" ! decodebin ! videoconvert ! video/x-raw,format=RGB ! appsink name=sink",
                full_path
            )
        };

        let pipeline_element = gst::parse::launch(&pipeline_str).expect("Pipeline xatosi");
        let pipeline = pipeline_element.dynamic_cast::<gst::Pipeline>().unwrap();
        let sink = pipeline.by_name("sink").unwrap().dynamic_cast::<gst_app::AppSink>().unwrap();

        sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                    let map = buffer.map_readable().map_err(|_| gst::FlowError::Error)?;
                    let caps = sample.caps().ok_or(gst::FlowError::Error)?;
                    let s = caps.structure(0).ok_or(gst::FlowError::Error)?;
                    
                    let width = s.get::<i32>("width").unwrap() as usize;
                    let height = s.get::<i32>("height").unwrap() as usize;

                    if let Ok(frame) = Array3::from_shape_vec((height, width, 3), map.as_slice().to_vec()) {
                        let _ = tx.send(frame);
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        pipeline.set_state(gst::State::Playing).expect("PLAYING holatiga o'tmadi");

        Camera { receiver: rx }
    }

    pub fn read(&self) -> Option<Array3<u8>> {
        self.receiver.try_recv().ok()
    }
}