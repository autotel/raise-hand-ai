use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_tensorflow_models_pose_detection::pose::{Keypoint, Pose};

/**
 * The frame pose contents, a list
 * of keypoints (joint positions)
 */
// TODO: Use Vec<Keypoint> instead of Pose, a Pose is a bit wasteful
pub type PoseRecord = Pose;

/**
 * A single frame of the whole body pose
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct KeypointRecordingFrame {
    pub keypoints: PoseRecord,
    pub timestamp: u128,
}


/**
 * An animation of the whole body poses over time
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct KeypointRecording {
    pub name: String,
    pub frames: Vec<KeypointRecordingFrame>,
    pub last_played_frame_time: u128,
    pub recording_started_time: u128
}

impl KeypointRecording {
    pub fn new(name: &str) -> KeypointRecording {
        KeypointRecording {
            name: name.to_string(),
            frames: Vec::new(),
            last_played_frame_time: 0,
            recording_started_time: 0,
        }
    }

    fn get_frames_between(&self, start: u128, end_incl: u128) -> Vec<KeypointRecordingFrame> {
        let mut acc: Vec<KeypointRecordingFrame> = Vec::new();

        for frame in self.frames.iter() {
            if (frame.timestamp > start)
            && (frame.timestamp <= end_incl) {
                acc.push(frame.clone());
            }
            // I save time assuming that frames come in chronol. order
            if frame.timestamp > end_incl { break; }
            continue;
        };
        acc        
    }

    pub fn playback_get_frames(&self, animation_frame: u128) -> Vec<KeypointRecordingFrame> {
        let last_frame_requested = self.last_played_frame_time;
        self.get_frames_between(last_frame_requested, animation_frame)
    }

    pub fn start_playback(&mut self) {
        self.last_played_frame_time = 0;
    }

    pub fn recording_add_frame(&mut self, keypoints: PoseRecord, timestamp: u128) {
        if timestamp < self.recording_started_time {
            panic!("Timestamp of add_frame is earlier than recording_started_time")
        }
        let time_since_recording_started = timestamp - self.recording_started_time;
        let pose_record = keypoints;
        let frame = KeypointRecordingFrame {
            keypoints: pose_record,
            timestamp,
        };
        self.frames.push(frame);
    }

    pub fn recording_start(&mut self, timestamp: u128) {
        self.recording_started_time = timestamp;
        self.frames = Vec::new();
    }

    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap_throw()
    }

    pub fn from_json(json: &str) -> KeypointRecording {
        serde_json::from_str(json).unwrap_throw()
    }
}
