use std::sync::Arc;

use material::Material;
use object::Object;
use renderer::start_render;
use scene::Scene;

mod material;
mod object;
mod renderer;
mod scene;
mod utils;

fn main() {
    let mut scene = Scene::new();

    scene.camera.origin = glam::vec3a(-2.0, 2.0, 1.0);
    scene.camera.lookat = glam::vec3a(0.0, 0.0, -1.0);
    scene.viewport_resize(1280, 720);
    scene.camera.update();

    scene.add_object(Object::Sphere {
        origin: glam::vec3a(0.0, -100.5, 0.0),
        radius: 100.0,
        material: Material::Diffuse {
            albedo: glam::vec3a(0.0, 0.7, 0.5),
        },
    });

    scene.add_object(Object::Sphere {
        origin: glam::vec3a(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Diffuse {
            albedo: glam::vec3a(0.5, 0.5, 0.8),
        },
    });

    scene.add_object(Object::Sphere {
        origin: glam::vec3a(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric {
            refraction_index: 1.5,
        },
    });

    scene.add_object(Object::Sphere {
        origin: glam::vec3a(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal {
            albedo: glam::vec3a(0.5, 0.4, 0.2),
            fuzziness: 0.0,
        },
    });

    scene.add_object(Object::Sphere {
        origin: glam::vec3a(-0.5, 0.0, -2.0),
        radius: 0.5,
        material: Material::Metal {
            albedo: glam::vec3a(0.3, 0.2, 0.5),
            fuzziness: 0.8,
        },
    });

    let start_time = std::time::Instant::now();
    let data = start_render(Arc::new(scene));

    for thread in data.threads {
        thread.join().unwrap();
    }

    data.image.lock().unwrap().save("output.png").unwrap();
    println!("\nSaved output.png.");
    println!("Took {:?} seconds.", start_time.elapsed().as_secs_f32());
}
