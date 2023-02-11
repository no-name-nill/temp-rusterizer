
pub struct Color(pub u8, pub u8, pub u8);

/*pub struct Color {
	R:u8,
	G:u8,
	B:u8
}
*/

impl Color{
	pub fn to_u32(&self) ->u32{
		let Color(R, G, B) = self;
		(256*256*(*R) as u32)+(256*(*G) as u32)+(*B) as u32
	}
}

pub trait Vectorx {
	fn swap<T:Vectorx>(a:T,  b:T);
	// normalize
	//dot
	//cross
	//mag
	//add
	//sub
}

pub struct vec2{	//<T>{
	pub x:f32,
	pub y:f32
}

pub struct vec3 {
	pub x:f32,
	pub y:f32,
	pub z:f32
}

impl vec2{ //Vectorx for
	
	pub fn swap(a:&mut vec2,  b:&mut vec2){
		let c = vec2{x:a.x, y:a.y};
		a.x = b.x; a.y = b.y;
		b.x = c.x; b.y = c.y;
	}
}

impl vec3{
	pub fn cross(a:vec3, b:vec3) -> vec3{
		let n = vec3{
			x: (a.y*b.z)-(b.y*a.z),
			y: (a.x*b.z)-(b.x*a.z),
			z: (a.x*b.y)-(b.x*a.y)
		};
		n
	}
}

pub fn lerp(x1:f32, y1:f32, x2:f32, y2:f32, x:f32) -> f32{ 
	let slope:f32 = (y2-y1) /(x2-x1);
	
	if (x2-x1) == 0.0 {return y2}

	let y = ((slope*(x-x1))+y1);
	y
}