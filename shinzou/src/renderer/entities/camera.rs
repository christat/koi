use ultraviolet::{projection, rotor::Rotor3, Mat4, Vec3, Vec4};
//----------------------------------------------------------------------------------------------------------------------

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    v_fov_rad: f32,
    aspect_ratio: f32,
    z_far: f32,
    z_near: f32,
}
//----------------------------------------------------------------------------------------------------------------------

impl Camera {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        v_fov_rad: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let up = Vec3::new(0.0, 1.0, 0.0);
        Self {
            position,
            direction,
            up,
            aspect_ratio,
            v_fov_rad,
            z_near,
            z_far,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn position(&self) -> Vec3 {
        self.position
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn view(&self) -> Mat4 {
        look_at(self.position, self.direction, self.up)
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn projection(&self) -> Mat4 {
        projection::perspective_vk(self.v_fov_rad, self.aspect_ratio, self.z_near, self.z_far)
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn translate(&mut self, translation: Vec3) {
        // TODO inefficient
        let local = look_at(self.position, self.direction, self.up).inversed();
        self.position += (local * Vec4::from(translation)).truncated();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn rotate(&mut self, rotor: Rotor3) {
        self.direction = (rotor * self.direction).normalized();
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

fn look_at(position: Vec3, direction: Vec3, up: Vec3) -> Mat4 {
    // Kindly borrowed from https://stackoverflow.com/a/21830596
    let z = direction;
    let mut y = up;
    let mut x = y.cross(z);
    y = z.cross(x);
    x.normalize();
    y.normalize();
    let w = Vec4::new(-x.dot(direction), -y.dot(direction), -z.dot(direction), 1.0);
    let rotation = Mat4::new(Vec4::from(x), Vec4::from(y), Vec4::from(z), w);
    let translation = Mat4::identity().translated(&position);
    rotation * translation
}
//----------------------------------------------------------------------------------------------------------------------
