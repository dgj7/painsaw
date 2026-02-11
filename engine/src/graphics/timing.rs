use crate::config::renderer_config::RendererConfig;
use std::cmp;
use std::time::Instant;

///
/// encapsulation of delta_time for use within the engine.
///
/// also exposes computation of fps/average, and fps cap.
///
pub struct EngineTiming {
    engine_start: Instant,
    frame_start: Instant,

    frame_count: u128,

    pub delta_time: f64,

    pub wait_between_frames: f64,
}

impl EngineTiming {
    ///
    /// create a new instance.
    ///
    pub fn new(rc: &RendererConfig) -> EngineTiming {
        EngineTiming {
            engine_start: Instant::now(),
            frame_start: Instant::now(),
            frame_count: 0,
            delta_time: 0.0,
            wait_between_frames: rc.fps_cap.map(|x| 1.0 / x as f64).unwrap_or(0.0),
        }
    }
}

impl EngineTiming {
    ///
    /// call at the beginning of frame rendering.
    ///
    /// should NOT include the entire loop (ie, message handling), even though that timing
    /// will be included in delta_time if end_frame is called correctly as well.
    ///
    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.frame_start).as_secs_f64();
        self.frame_start = now;
    }

    ///
    /// call at the end of frame rendering.
    ///
    /// shouldn't include the entire loop; only concludes rendering calls.
    ///
    pub fn end_frame(&mut self) {
        self.frame_count += 1;
    }

    ///
    /// compute the frames per second.
    ///
    pub fn compute_fps(&self) -> u32 {
        let fps_float = 1.0 / self.delta_time;
        cmp::min(fps_float as u32, 9999)
    }

    ///
    /// compute the average frames per second.
    ///
    pub fn compute_avg_fps(&self) -> u32 {
        let now = Instant::now();
        let high_res_run_time = now.duration_since(self.engine_start).as_secs_f64();
        let fps_float = self.frame_count as f64 / high_res_run_time;
        cmp::min(fps_float as u32, 9999)
    }

    ///
    /// determine if the desired time between frames has been reached.
    ///
    /// this is an implementation of fps cap.
    ///
    pub fn is_ok_to_render(&self) -> bool {
        let elapsed = Instant::now()
            .duration_since(self.frame_start)
            .as_secs_f64();
        elapsed >= self.wait_between_frames
    }
}
