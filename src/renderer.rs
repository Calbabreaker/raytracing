use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{material::HitInfo, object::Object, scene::Scene, utils::Ray};

pub struct RenderingData {
    pub image: Arc<Mutex<image::RgbImage>>,
    pub threads: Vec<std::thread::JoinHandle<()>>,
}

pub fn start_render(scene: Arc<Scene>) -> RenderingData {
    println!("Rendering with {} samples, {} threads...", scene.samples_per_pixel, scene.render_threads);

    let screen_dim = glam::uvec2(scene.width, scene.height);
    let max_tiles = (screen_dim.as_vec2() / scene.render_tile_size as f32)
        .ceil()
        .as_uvec2();
    let total_tiles = (max_tiles.x * max_tiles.y) as u32;

    let tile_index_mut = Arc::new(Mutex::new(0));
    let image = Arc::new(Mutex::new(image::RgbImage::new(scene.width, scene.height)));
    let scene = Arc::new(scene);
    let mut threads = Vec::new();

    for _ in 0..scene.render_threads {
        let image = Arc::clone(&image);
        let scene = Arc::clone(&scene);
        let tile_index_mut = Arc::clone(&tile_index_mut);

        threads.push(std::thread::spawn(move || loop {
            let tile_index = *tile_index_mut.lock().unwrap();
            if tile_index > total_tiles {
                break;
            }

            *tile_index_mut.lock().unwrap() += 1;

            let start_tile = glam::uvec2(tile_index % max_tiles.x, tile_index / max_tiles.x);
            let start_pos = start_tile * scene.render_tile_size;
            let end_pos = ((start_tile + 1) * scene.render_tile_size).min(screen_dim);

            print!(
                "\rRendering tile: {}/{} at {}     ",
                tile_index, total_tiles, start_pos
            );
            std::io::stdout().flush().unwrap();

            for x in start_pos.x..end_pos.x {
                for y in start_pos.y..end_pos.y {
                    let pixel = sample_pixel(x, scene.height - y, &scene);
                    image.lock().unwrap().put_pixel(x, y, pixel);
                }
            }
        }))
    }

    RenderingData { image, threads }
}

fn sample_pixel(x: u32, y: u32, scene: &Scene) -> image::Rgb<u8> {
    let mut color = glam::Vec3A::ZERO;
    for _ in 0..scene.samples_per_pixel {
        let u = (x as f32 + fastrand::f32()) / (scene.width - 1) as f32;
        let v = (y as f32 + fastrand::f32()) / (scene.height - 1) as f32;

        let mut ray = scene.camera.get_ray(u, v);
        color += ray_color(&scene.objects, &mut ray, scene.max_ray_bounces);
    }

    // take the average of all the slightly differing rays of all the samples
    color /= scene.samples_per_pixel as f32;
    let r = (255.0 * color.x.sqrt()) as u8;
    let g = (255.0 * color.y.sqrt()) as u8;
    let b = (255.0 * color.z.sqrt()) as u8;
    image::Rgb([r, g, b])
}

fn ray_color(objects: &[Object], ray: &mut Ray, max_bounces: u32) -> glam::Vec3A {
    let mut attenuation = glam::Vec3A::ONE;
    let mut bounces_left = max_bounces;

    loop {
        if bounces_left == 0 {
            return glam::Vec3A::ZERO;
        }

        let mut hit_info = HitInfo::empty();
        if ray_cast(ray, 0.01, 1000.0, objects, &mut hit_info) {
            if !hit_info
                .material
                .unwrap()
                .scatter(ray, &hit_info, &mut attenuation)
            {
                return glam::Vec3A::ZERO;
            }
        } else {
            // add skybox light color
            const SKYBOX_COLOR_TOP: glam::Vec3A = glam::const_vec3a!([0.5, 0.7, 1.0]);
            const SKYBOX_COLOR_BOTTOM: glam::Vec3A = glam::const_vec3a!([1.0, 1.0, 1.0]);

            let color_scale = (ray.direction.y + 1.0) / 2.0;
            attenuation *= SKYBOX_COLOR_BOTTOM.lerp(SKYBOX_COLOR_TOP, color_scale);
            break;
        }

        bounces_left -= 1;
    }

    attenuation
}

fn ray_cast<'a>(
    ray: &Ray,
    dist_min: f32,
    dist_max: f32,
    objects: &'a [Object],
    hit_info: &mut HitInfo<'a>,
) -> bool {
    let mut closest_dist = dist_max;
    for object in objects {
        if object.ray_intersect(ray, dist_min, closest_dist, hit_info) {
            closest_dist = hit_info.dist;
        }
    }

    closest_dist != dist_max
}
