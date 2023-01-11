use gstreamer::{prelude::*, Element};

use log::info;
use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use crate::config::Config;

#[allow(dead_code)]
pub enum Message {
    Update,
    Quit,
}

pub fn spawn_streamer(
    rx: Receiver<Message>,
    config: Config,
) -> JoinHandle<()> {
    let pipeline_str: String = format!(
        "uridecodebin uri=file://{0} \
            ! videoscale \
            ! imagefreeze \
            ! videoconvert
            ! video/x-raw, framerate={1}, format=NV12, width={2}, height={3}
            ! queue
            ! x264enc threads=0 bitrate={4} tune=zerolatency key-int-max=30
            ! h264parse
            ! queue
            ! flvmux name=flvmux
            ! queue
            ! rtmpsink location=rtmp://{5}",
        config.image_file_path,
        config.frame_rate,
        config.stream_width,
        config.stream_height,
        config.bit_rate,
        config.rtmp_endpoint,
    );

    thread::spawn(move || {
        gstreamer::init().expect("initialization failed");
        let mut pipeline: Option<Element> = None;
        info!("gstreamer initialized");

        loop {
            match rx.recv().unwrap() {
                Message::Update => {
                    info!("update received, restarting pipeline ...");
                    let new_pipeline =
                        gstreamer::parse_launch(&pipeline_str).expect("failed to parse pipeline");
                    if let Some(pipeline) = pipeline {
                        pipeline
                            .set_state(gstreamer::State::Null)
                            .expect("failed to set the state to Null");
                    }
                    new_pipeline
                        .set_state(gstreamer::State::Playing)
                        .expect("failed to set the state to Playing");
                    pipeline = Some(new_pipeline);
                    info!("pipeline restarted");
                }
                Message::Quit => {
                    // NOT IMPLEMENTED
                    info!("exiting");
                    if let Some(pipeline) = pipeline {
                        pipeline
                            .set_state(gstreamer::State::Null)
                            .expect("failed to set the state to Null");
                    }
                    break;
                }
            };
        }
    })
}
