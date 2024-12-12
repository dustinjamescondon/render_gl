pub type Point2f = nalgebra::Point2<f32>;
pub type Vector2f = nalgebra::Vector2<f32>;
pub type Vector3f = nalgebra::Vector3<f32>;

const FOV_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
const NEAR: f32 = 0.1_f32;
const FAR: f32 = 5000_f32;
const Z_DISTANCE: f32 = 800_f32;

/// Defines a 2D camera--i.e. a camera that is fixed looking in the Z direction
pub struct Camera {
    pub focus_pt: Point2f,
    pub width: u32,
    pub height: u32,
    pub rotation: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            focus_pt: Point2f::origin(),
            rotation: 0.0,
            zoom: 0_f32,
            width: 1,
            height: 1,
        }
    }
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self { 

        let mut cam = Camera {
            width,
            height,
            ..Default::default()
        };
		cam.resize(width, height);
		cam
    }

	pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
		self.width = width;
		self.height = height;
		self
	}

	pub fn zoom(&mut self, dist: f32) {
		self.zoom += dist;
	}

	pub fn set_zoom(&mut self, level: f32) {
		self.zoom = level
	}

	pub fn shift_focus(&mut self, delta: Vector2f) {
		self.look_at(self.focus_pt + delta, 0_f32);
	}

	/// Pans and rotates the camera to look at given world point
	pub fn look_at(&mut self, pt: Point2f, angle: f32) {
		
		// set's view s.t.
		// 		view * pt = <0,0,0>
		self.focus_pt = pt;
		self.rotation = angle;
	}

	fn z(&self) -> f32 {
		Z_DISTANCE- self.zoom
	}

	pub fn view(&self) -> nalgebra_glm::Mat4 {
		let rotation = nalgebra_glm::rotation(self.rotation, &Vector3f::z_axis());
        let offset = nalgebra_glm::Vec3::new(-self.focus_pt.x, -self.focus_pt.y, -self.z());
        rotation * nalgebra_glm::translation(&offset)
    }

	pub fn view_inverse(&self) -> nalgebra_glm::Mat4 {
		let rotation_inv = nalgebra_glm::rotation(-self.rotation, &Vector3f::z_axis());
        let offset = nalgebra_glm::Vec3::new(self.focus_pt.x, self.focus_pt.y, self.z());
        nalgebra_glm::translation(&offset) * rotation_inv
    }

    pub fn projection(&self) -> nalgebra_glm::Mat4 {	
		//nalgebra_glm::perspective(1.0, FOV_ANGLE, 0.1_f32, 1000_f32);
		perspective(FOV_ANGLE, NEAR, FAR)
    }

	pub fn orthographic(&self) -> nalgebra_glm::Mat4 {
		orthographic(self.width, self.height)
	}

	pub fn projection_inverse(&self) -> nalgebra_glm::Mat4 {	
		//nalgebra_glm::perspective(1.0, FOV_ANGLE, 0.1_f32, 1000_f32);
		inverse_perspective(FOV_ANGLE, NEAR, FAR)
    }

	pub fn world_to_screen(&self, pt: Point2f) -> Point2f {
		let homo = nalgebra_glm::Vec4::new(pt.x, pt.y, 0_f32, 1.0_f32);
		let pt_ = self.projection() * self.view() * homo;
		Point2f::new(pt_.x, pt_.y) / pt_.w
	}

    pub fn screen_to_world(&self, pt: Point2f) -> Point2f {
		let v = nalgebra_glm::Vec4::new(pt.x, pt.y, perspective_z(NEAR, FAR, -self.z()), 1_f32);
		let world_pt = self.view_inverse() * self.projection_inverse() * v;
		Point2f::new(world_pt.x, world_pt.y) / world_pt.w
    }

	/// The dimensions of the world plane at the given z-level
	pub fn world_plane_dim(&self, _z: f32) -> Vector2f {
		let left_screen = Point2f::new(-1_f32, 0_f32);
		let right_screen = Point2f::new(1_f32, 0_f32);
		let bottom_screen = Point2f::new(0_f32, -1_f32);
		let top_screen = Point2f::new(0_f32, 1_f32);

		let l_world = self.screen_to_world(left_screen);
		let r_world = self.screen_to_world(right_screen);
		let b_world = self.screen_to_world(bottom_screen);
		let t_world = self.screen_to_world(top_screen);

		Vector2f::new((r_world - l_world).norm(), (t_world - b_world).norm())
	}
}


pub fn orthographic(w: u32, h: u32) -> nalgebra_glm::Mat4 {
	nalgebra_glm::ortho(0_f32, w as f32, 0_f32, h as f32, -10_f32, 10_f32)
}

pub fn perspective(fov_angle: f32, near: f32, far: f32) -> nalgebra_glm::Mat4 {
	let half_angle = 0.5_f32 * fov_angle;
	let right = f32::tan(half_angle) * near;
	let top = f32::tan(half_angle) * near;
	nalgebra_glm::Mat4::new(
		near / right, 0_f32, 0_f32, 0_f32,
		0_f32, near / top, 0_f32, 0_f32,
		0_f32, 0_f32, - (far + near) / (far - near), - 2_f32 * (far * near) / (far - near),
		0_f32, 0_f32, -1_f32, 0_f32
	)
}

pub fn perspective_z(near: f32, far: f32, target_z: f32) -> f32 {
	/* 
	let numerator = -(2_f32 * far * near) + (target_z * (far - near));
	let denominator = far + near;
	let prescale = numerator / denominator;
	prescale / target_z
	*/
	let v = nalgebra_glm::Vec4::new(0_f32, 0_f32, target_z, 1_f32);
	let u = perspective(FOV_ANGLE, near, far) * v;
	u.z / u.w
}

pub fn inverse_perspective(fov_angle: f32, near: f32, far: f32) -> nalgebra_glm::Mat4 {

	let half_angle = 0.5_f32 * fov_angle;
	let right = f32::tan(half_angle) * near;
	let top = f32::tan(half_angle) * near;
	nalgebra_glm::Mat4::new(
		right / near, 0_f32, 0_f32, 0_f32,
		0_f32, top / near, 0_f32, 0_f32,
		0_f32, 0_f32, 0_f32, -1_f32,
		0_f32, 0_f32, (far - near) / (-2_f32 * near * far), (far + near) / (2_f32 * far * near)
	)
}

#[cfg(test)]
mod test {
    use crate::render_system::camera::*;

    use super::perspective;

	#[test]
	fn perspective_inverse() {
		let near = NEAR;
		let far = FAR;
		let m = perspective(super::FOV_ANGLE, near, far);
		let m_inv = inverse_perspective(super::FOV_ANGLE, near, far);

		let result = m * m_inv;
		let expected = nalgebra_glm::Mat4::identity();

		let are_eq = nalgebra_glm::Mat4::relative_eq(&result, &expected, 1.0e-5_f32, 0.01_f32);
		assert!(are_eq);
	}

	#[test]
	fn sandbox() {
		let near = NEAR;
		let far = FAR;
		let perspective = perspective(super::FOV_ANGLE, near, far);
		let pt = nalgebra_glm::Vec4::new(100_f32, -150_f32, -800_f32, 1_f32);

		let result = perspective * pt;
		let scaled = result / result.w;

		let x_is_normalizd = f32::abs(scaled.x) <= 1_f32;
		let y_is_normalizd = f32::abs(scaled.y) <= 1_f32;
		let z_is_normalizd = f32::abs(scaled.z) <= 1_f32;

		//let are_eq = nalgebra_glm::Mat4::relative_eq(&result, &expected, 1.0e-5_f32, 0.01_f32);
		assert!(x_is_normalizd);
		assert!(y_is_normalizd);
		assert!(z_is_normalizd);
	}


	#[test]
	fn screen_to_world_and_back_again_2_electric_boogaloo() {
		let cam = Camera::new(100, 100);

		let world_pt = Point2f::new(100_f32, 100_f32);
		let screen_pt = cam.world_to_screen(world_pt);
		let world_pt_2 = cam.screen_to_world(screen_pt);

		assert!(Vector2f::relative_eq(&Vector2f::zeros(), &(world_pt - world_pt_2), 1.0e-3, 0.1)); // Note the low accuracy
	}
	
	#[test]
	fn screen_to_world_and_back_again() {
		//let cam = Camera::new(100, 100);

		let world_pt = nalgebra_glm::Vec4::new(100_f32, -150_f32, -800_f32, 1_f32);
		let mut screen_pt = perspective(FOV_ANGLE, NEAR, FAR) * world_pt;
		screen_pt /= screen_pt.w;
		let mut result = inverse_perspective(FOV_ANGLE, NEAR, FAR) * screen_pt;
		result /= result.w;
		assert!(eq_eps(result, world_pt, 1.0)); // Note the low accuracy
	}

	#[test]
	fn z_mapping() {
		let world_pt = nalgebra_glm::Vec4::new(100_f32, -150_f32, -800_f32, 1_f32);
		let mut screen_pt = perspective(FOV_ANGLE, NEAR, FAR) * world_pt;
		screen_pt /= screen_pt.w;
		let result = perspective_z(NEAR, FAR, -800_f32);
		assert!(f32::abs(result - screen_pt.z) < 1.0e-2_f32);
	}

	fn eq_eps(a: nalgebra_glm::Vec4, b: nalgebra_glm::Vec4, eps: f32) -> bool {
		let diff = a - b;
		(f32::abs(diff.y) < eps) &&
		(f32::abs(diff.z) < eps) &&
		(f32::abs(diff.x) < eps) &&
		(f32::abs(diff.w) < eps)
	}
}