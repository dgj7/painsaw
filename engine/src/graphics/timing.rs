use std::cmp;
use std::time::Instant;
use crate::config::renderer_config::RendererConfig;

pub struct EngineTiming {
    engine_start: Instant,
    frame_start: Instant,
    
    frame_count: u128,
    
    pub delta_time: f64,

    pub wait_between_frames: f64,
}

impl EngineTiming {
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
    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.frame_start).as_secs_f64();
        self.frame_start = now;
    }
    
    pub fn end_frame(&mut self) {
        self.frame_count += 1;
    }
    
    pub fn compute_fps(&self) -> u32 {
        let fps_float = 1.0 / self.delta_time;
        cmp::min(fps_float as u32, 9999)
    }
    
    pub fn compute_avg_fps(&self) -> u32 {
        let now = Instant::now();
        let high_res_run_time = now.duration_since(self.engine_start).as_secs_f64();
        let fps_float = self.frame_count as f64 / high_res_run_time;
        cmp::min(fps_float as u32, 9999)
    }

    pub fn should_wait_to_render(&self) -> bool {
        let elapsed = Instant::now().duration_since(self.frame_start).as_secs_f64();
        elapsed < self.wait_between_frames
    }
}
