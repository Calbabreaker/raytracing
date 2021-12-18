use crate::utils::{near_zero, random_unit_vector, reflect, reflectance, refract, Ray};

#[derive(Copy, Clone)]
pub enum Material {
    Diffuse { albedo: glam::Vec3A },
    Metal { albedo: glam::Vec3A, fuzziness: f32 },
    Dielectric { refraction_index: f32 },
}

impl Material {
    pub fn diffuse(albedo: glam::Vec3A) -> Self {
        Self::Diffuse { albedo }
    }

    pub fn metal(albedo: glam::Vec3A, fuzziness: f32) -> Self {
        Self::Metal { albedo, fuzziness }
    }

    pub fn dieletric(refraction_index: f32) -> Self {
        Self::Dielectric { refraction_index }
    }

    // returns true if valid scatter
    pub fn scatter(
        &self,
        ray: &mut Ray,
        hit_info: &HitInfo,
        attenuation: &mut glam::Vec3A,
    ) -> bool {
        match self {
            Material::Diffuse { albedo } => {
                let mut direction = hit_info.normal + random_unit_vector();

                if near_zero(direction) {
                    direction = hit_info.normal;
                }

                *ray = Ray::new(hit_info.point, direction);
                *attenuation *= *albedo;
            }

            Material::Metal { albedo, fuzziness } => {
                let direction = reflect(ray.direction, hit_info.normal);
                let fuzz_vec = *fuzziness * random_unit_vector();

                *ray = Ray::new(hit_info.point, direction + fuzz_vec);
                *attenuation *= *albedo;
                return ray.direction.dot(hit_info.normal) > 0.0;
            }

            Material::Dielectric { refraction_index } => {
                let refraction_ratio = if hit_info.is_front_face {
                    refraction_index.recip()
                } else {
                    *refraction_index
                };

                let cos_theta = (-ray.direction).dot(hit_info.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let should_reflect = reflectance(cos_theta, refraction_ratio) > fastrand::f32();

                let direction = if cannot_refract || should_reflect {
                    reflect(ray.direction, hit_info.normal)
                } else {
                    refract(ray.direction, hit_info.normal, refraction_ratio)
                };

                *ray = Ray::new(hit_info.point, direction);
            }
        }

        true
    }
}

pub struct HitInfo<'a> {
    pub point: glam::Vec3A,
    pub normal: glam::Vec3A,
    pub dist: f32,
    pub is_front_face: bool,
    pub material: Option<&'a Material>,
}

impl HitInfo<'_> {
    pub fn empty() -> HitInfo<'static> {
        HitInfo {
            point: glam::Vec3A::ZERO,
            normal: glam::Vec3A::ZERO,
            dist: 0.0,
            is_front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: glam::Vec3A) {
        self.is_front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
