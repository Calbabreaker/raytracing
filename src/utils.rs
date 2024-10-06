pub struct Ray {
    pub origin: glam::Vec3A,
    pub direction: glam::Vec3A,
}

impl Ray {
    pub fn new(origin: glam::Vec3A, direction: glam::Vec3A) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, dist: f32) -> glam::Vec3A {
        self.origin + dist * self.direction
    }
}

#[derive(Default)]
pub struct Camera {
    pub origin: glam::Vec3A,
    pub lookat: glam::Vec3A,
    pub lens_radius: f32,
    pub fov: f32,
    pub aspect_ratio: f32,

    lower_left_corner: glam::Vec3A,
    horizontal: glam::Vec3A,
    vertical: glam::Vec3A,
    u: glam::Vec3A,
    v: glam::Vec3A,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 40.,
            lookat: glam::Vec3A::X,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let viewport_height = (self.fov.to_radians() / 2.0).tan() * 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;

        let w = self.origin - self.lookat;
        let focus_dist = w.length();
        let w_norm = w / focus_dist;
        self.u = glam::Vec3A::Y.cross(w_norm).normalize();
        self.v = w_norm.cross(self.u);

        self.horizontal = viewport_width * self.u * focus_dist;
        self.vertical = viewport_height * self.v * focus_dist;
        self.lower_left_corner = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - w;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let random_disk = self.lens_radius * random_in_unit_disk();
        let offset = self.u * random_disk.x + self.v * random_disk.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

pub fn random_f32r(min: f32, max: f32) -> f32 {
    min + (max - min) * fastrand::f32()
}

pub fn random_in_unit_disk() -> glam::Vec2 {
    loop {
        let point = glam::vec2(random_f32r(-1.0, 1.0), random_f32r(-1.0, 1.0));
        if point.length_squared() < 1.0 {
            return point;
        }
    }
}

pub fn random_unit_vector() -> glam::Vec3A {
    glam::vec3a(
        random_f32r(-1.0, 1.0),
        random_f32r(-1.0, 1.0),
        random_f32r(-1.0, 1.0),
    )
    .normalize()
}

pub fn near_zero(vector: glam::Vec3A) -> bool {
    const EPSILON: f32 = 0.01;
    vector.length_squared() < EPSILON.powi(2)
}

pub fn reflect(vector: glam::Vec3A, normal: glam::Vec3A) -> glam::Vec3A {
    vector - 2.0 * vector.dot(normal) * normal
}

pub fn refract(vector: glam::Vec3A, normal: glam::Vec3A, etai_over_etat: f32) -> glam::Vec3A {
    let cos_theta = (-vector).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (vector + cos_theta * normal);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -normal;
    r_out_perp + r_out_parallel
}

// Uses Schlick's approximation for reflectance
pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
