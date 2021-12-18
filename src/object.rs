use crate::{
    material::{HitInfo, Material},
    utils::Ray,
};

#[derive(PartialEq)]
pub enum ObjectData {
    Sphere { origin: glam::Vec3A, radius: f32 },
}

pub struct Object {
    material: Material,
    data: ObjectData,
}

impl Object {
    pub fn sphere(origin: glam::Vec3A, radius: f32, material: Material) -> Self {
        Self {
            data: ObjectData::Sphere { origin, radius },
            material,
        }
    }

    // returns true if ray hits object
    pub fn ray_intersect<'a>(
        &'a self,
        ray: &Ray,
        dist_min: f32,
        dist_max: f32,
        hit_info: &mut HitInfo<'a>,
    ) -> bool {
        match self.data {
            ObjectData::Sphere { origin, radius } => {
                let oc = ray.origin - origin;
                let a = ray.direction.length_squared();
                let half_b = oc.dot(ray.direction);
                let c = oc.length_squared() - radius.powi(2);
                let discriminant = half_b.powi(2) - a * c;

                if discriminant < 0.0 {
                    return false;
                }

                let sqrtd = discriminant.sqrt();

                // find the nearest distance that lies in the acceptable range.
                hit_info.dist = (-half_b - sqrtd) / a;
                if hit_info.dist < dist_min || hit_info.dist > dist_max {
                    hit_info.dist = (-half_b + sqrtd) / a;
                    if hit_info.dist < dist_min || hit_info.dist > dist_max {
                        return false;
                    }
                }

                hit_info.point = ray.at(hit_info.dist);
                let outward_normal = (hit_info.point - origin) / radius;
                hit_info.set_face_normal(ray, outward_normal);
            }
        }

        hit_info.material = Some(&self.material);
        return true;
    }
}
