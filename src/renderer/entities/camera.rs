use ultraviolet::{projection, rotor::Rotor3, Mat4, Vec3};
//----------------------------------------------------------------------------------------------------------------------

pub struct Camera {
    position: Vec3,
    direction: Vec3,

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
        Self {
            position,
            direction,
            aspect_ratio,
            v_fov_rad,
            z_near,
            z_far,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn view(&self) -> Mat4 {
        Mat4::from_translation(self.position)
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn projection(&self) -> Mat4 {
        projection::perspective_vk(self.v_fov_rad, self.aspect_ratio, self.z_near, self.z_far)
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn rotate(&mut self, rotor: Rotor3) {
        self.direction = rotor * self.direction;
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
