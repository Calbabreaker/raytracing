use crate::{object::Object, utils::Camera};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,

    pub samples_per_pixel: u32,
    pub max_ray_bounces: u32,
    pub render_threads: usize,
    pub render_tile_size: u32,
    pub width: u32,
    pub height: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Camera::new(),
            objects: Vec::new(),
            samples_per_pixel: 250,
            max_ray_bounces: 32,
            render_tile_size: 128,
            render_threads: get_threads().unwrap_or(8),
            width: 0,
            height: 0,
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn viewport_resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.camera.aspect_ratio = width as f32 / height as f32;
    }
}

fn get_threads() -> Option<usize> {
    Some(std::thread::available_parallelism().ok()?.get())
}
