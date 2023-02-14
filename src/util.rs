use std::ops::{Add, Sub};
use std::f32;


pub struct Color(pub u8, pub u8, pub u8, pub f32);

/*pub struct Color {
	R:u8,
	G:u8,
	B:u8
}
*/

impl Color{
	pub fn to_u32(&self) ->u32{
		let Color(R, G, B, A) = self;
		(256*256*(*R) as u32)+(256*(*G) as u32)+(*B) as u32
	}

	pub fn mix(c1:&Color, c2:&Color, t:f64) -> Color{
		let Color(R1, G1, B1, A1) = c1;
		let Color(R2, G2, B2, A2) = c2;
		//c1+(c2-c1)*t;
		let c3:Color = Color((*R1 as f64+(((*R2) as f64 - (*R1) as f64)*t))as u8,
			(*G1 as f64+(((*G2) as f64- (*G1) as f64)as f64*t))as u8,
			(*B1 as f64+(((*B2) as f64- (*B1) as f64)as f64*t))as u8, 1.0);
		c3
	}

	pub fn swap(c1:&mut Color, c2:&mut Color){
		let R1 = c1.0;
		let R2 = c2.0;
		let G1 = c1.1;
		let G2 = c2.1;
		let B1 = c1.2;
		let B2 = c2.2;
		let A1 = c1.3;
		let A2 = c2.3;
		c1.0 = R2;
		c2.0 = R1;
		c1.1 = G2;
		c2.1 = G1;
		c1.2 = B2;
		c2.2 = B1;
		c1.3 = A2;
		c2.3 = A1;
	}
	pub fn copy(&self)->Color{
		let Color(R, G, B, A) = *self;
		Color(R, G, B, A)
	}
}

pub trait Vectorx: Add + Sub + Sized { //useless
	//fn swap<T:Vectorx>(a:T,  b:T);
	//normalize
	//dot
	//cross
	//mag
}

pub struct vec2{
	pub x:f32,
	pub y:f32
}

pub struct vec3 {
	pub x:f32,
	pub y:f32,
	pub z:f32
}

impl vec2{
	
	pub fn swap(a:&mut vec2,  b:&mut vec2){
		let c = vec2{x:a.x, y:a.y};
		a.x = b.x; a.y = b.y;
		b.x = c.x; b.y = c.y;
	}

	pub fn to_vec3(&self) -> vec3{
		vec3{
			x: self.x,
			y: self.y,
			z: 0.0
		}
	}

	//pub fn copy(&self)->vec2{}
}


impl Add for &vec3 {
	type Output = vec3;

	fn add(self, other:Self) -> vec3{
		vec3{
			x: self.x+other.x,
			y: self.y+other.y,
			z: self.z+other.z
		}
	}
}

impl Sub for &vec3 {
	type Output = vec3;

	fn sub(self, other:Self) -> vec3{
		vec3{
			x: self.x-other.x,
			y: self.y-other.y,
			z: self.z-other.z
		}
	}
}

impl vec3{

	pub fn to_vec2(&self) -> vec2{
		vec2{
			x: self.x,
			y: self.y
		}
	}

	pub fn cross(a:&vec3, b:&vec3) -> vec3{
		let n = vec3{
			x: (a.y*b.z)-(b.y*a.z),
			y: (a.x*b.z)-(b.x*a.z),
			z: (a.x*b.y)-(b.x*a.y)
		};
		n
	}

	pub fn dot(a:&vec3, b:&vec3)->f32{ //or f64
		let dot = a.x*b.x + a.y*b.y + a.z*b.z;
		dot
	}

	pub fn normalize(a:&vec3) -> vec3{
		let mag = vec3::magnitude(&a);
		vec3{
			x: (a.x/mag),
			y: (a.y/mag),
			z: (a.z/mag)
		}
	}

	fn magnitude(a:&vec3) -> f32{
		let mag = ((a.x*a.x)+(a.y*a.y)+(a.z*a.z)).sqrt();
		mag
	}
	//pub fn copy(&self)->vec2{}

}

pub fn lerp(x1:f32, y1:f32, x2:f32, y2:f32, x:f32) -> f32{ 
	let slope:f32 = (y2-y1) /(x2-x1);
	
	if (x2-x1) == 0.0 {return y2}

	let y = ((slope*(x-x1))+y1);
	y
}


pub struct vertex {
	pub pos: vec3,
	pub color: Option<Color>
	//normal
}