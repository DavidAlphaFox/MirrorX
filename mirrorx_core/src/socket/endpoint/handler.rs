use super::{
    endpoint::EndPoint,
    message::{
        AudioFrame, DisplayInfo, GetDisplayInfoRequest, GetDisplayInfoResponse,
        StartMediaTransmissionResponse, VideoFrame,
    },
};
use crate::{error::MirrorXError, socket::endpoint::message::StartMediaTransmissionRequest};

pub async fn handle_get_display_info_request(
    endpoint: &EndPoint,
    req: GetDisplayInfoRequest,
) -> Result<GetDisplayInfoResponse, MirrorXError> {
    let monitors = crate::component::monitor::get_active_monitors()?;

    // todo: monitor and display_info has same memory layout, use memory block copy?
    let mut displays = Vec::new();

    for monitor in monitors {
        displays.push(DisplayInfo {
            id: monitor.id,
            name: monitor.name,
            refresh_rate: monitor.refresh_rate,
            width: monitor.width,
            height: monitor.height,
            is_primary: monitor.is_primary,
            screen_shot: monitor.screen_shot,
        });
    }

    Ok(GetDisplayInfoResponse { displays })
}

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    let fps = req.expect_fps;
    let display_id = req.expect_display_id;

    endpoint.start_audio_capture_process().await?;
    endpoint.start_video_capture_process().await?;

    let reply = StartMediaTransmissionResponse {
        os_name: crate::constants::OS_TYPE
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("unknown")),
        os_version: crate::constants::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("unknown")),
        screen_width: 1920,
        screen_height: 1080,
        video_type: String::from("todo"),
        audio_type: String::from("todo"),
    };

    Ok(reply)
}

pub async fn handle_video_frame(
    endpoint: &EndPoint,
    video_frame: VideoFrame,
) -> Result<(), MirrorXError> {
    endpoint.enqueue_video_frame(video_frame).await;
    Ok(())
}

pub async fn handle_audio_frame(
    endpoint: &EndPoint,
    audio_frame: AudioFrame,
) -> Result<(), MirrorXError> {
    endpoint.enqueue_audio_frame(audio_frame).await;
    Ok(())
}
