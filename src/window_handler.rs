use minifb::{Key, Window, WindowOptions};
use crate::util::*;

pub struct WindowHandler {
	pub window: Window,
	pub buffer: Vec<u32>,
	pub win_WIDTH: usize,
	pub win_HEIGHT: usize,
	pub current_color: u32

}

impl WindowHandler{
	pub fn new(WIDTH:usize, HEIGHT:usize)->WindowHandler{
		let mut window = Window::new(
			"Rusterizer!!",
			WIDTH,
			HEIGHT,
			WindowOptions::default()
			).unwrap();

		window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

		let mut buffer: Vec<u32> = vec![0;WIDTH*HEIGHT];

		WindowHandler{window: window,
			buffer: buffer,
			win_WIDTH: WIDTH,
			win_HEIGHT: HEIGHT,
			current_color: Color(255,255,255,1.0).to_u32()//WHITE(255,255,255) AND BLACK(0)
		}
	}

	pub fn clear(&mut self){
		for i in self.buffer.iter_mut(){
			*i = 0;
		}
		self.window.update_with_buffer(&self.buffer, self.win_WIDTH, self.win_HEIGHT).unwrap();
	}

	pub fn render(&mut self){
		self.window.update_with_buffer(&self.buffer, self.win_WIDTH, self.win_HEIGHT).unwrap();
	}

	pub fn set_color(&mut self, color:Color){
		self.current_color = color.to_u32();
	}

	pub fn putPixel(&mut self, x:u16, y:u16){
		self.buffer[(x as u32 +y as u32 *self.win_WIDTH as u32)as usize] = self.current_color;
	}

	pub fn put_pixel_with_color(&mut self, x:u16, y:u16, color:Color){
		self.buffer[(x as u32 + y as u32 *self.win_WIDTH as u32)as usize] = color.to_u32();
	}
	
	pub fn draw_line(&mut self, x1:u16, y1:u16, x2:u16, y2:u16){
		let diffx = if x2>x1 {x2-x1+1} else {x1-x2+1};
		let diffy = if y2>y1 {y2-y1+1} else {y1-y2+1};

		if diffx > diffy{
			for i in 0..diffx{
				let x3 = if x2>x1 {x1+i} else {x1-i};
				let y3 = lerp(x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}
		else{
			for i in 0..diffy{
				let y3 = if y2>y1 {y1+i} else {y1-i};
				let x3 = lerp(y1 as f32, x1 as f32, y2 as f32, x2 as f32, y3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}

	}


	pub fn draw_triangle(&mut self, style:&str, p1: &vec2, p2: &vec2, p3: &vec2){
		if style == "WIREFRAME"{
			self.draw_line(p1.x as u16, p1.y as u16, p2.x as u16, p2.y as u16);
			self.draw_line(p2.x as u16, p2.y as u16, p3.x as u16, p3.y as u16);
			self.draw_line(p1.x as u16, p1.y as u16, p3.x as u16, p3.y as u16);
		}
		else if style == "FILL"{
			let mut sp1 = vec2{x:p1.x, y:p1.y};
			let mut sp2 = vec2{x:p2.x, y:p2.y};
			let mut sp3 = vec2{x:p3.x, y:p3.y};
			if sp1.y > sp2.y{vec2::swap(&mut sp1, &mut sp2);}
			if sp1.y > sp3.y{vec2::swap(&mut sp1, &mut sp3);}
			if sp2.y > sp3.y{vec2::swap(&mut sp2, &mut sp3);}

			if sp1.y == sp2.y && sp1.x>sp2.x{vec2::swap(&mut sp1, &mut sp2);}

			for i in 0..(sp2.y-sp1.y+1.0) as u16{
				let i = f32::from(i);
				//lerp line 1-2
				let x1 = lerp(sp1.y, sp1.x, sp2.y, sp2.x, sp1.y+i);
				//lerp line 1-3
				let x2 = lerp(sp1.y, sp1.x, sp3.y, sp3.x, sp1.y+i);
				//draw
				self.draw_line(x1 as u16, (sp1.y+i) as u16, x2 as u16, (sp1.y+i) as u16);
			}
			for i in 0..(sp3.y-sp2.y+1.0) as u16{
				let i = f32::from(i);
				//lerp line 1-2
				let x1 = lerp(sp2.y, sp2.x, sp3.y, sp3.x, sp2.y+i);
				//lerp line 1-3
				let x2 = lerp(sp1.y, sp1.x, sp3.y, sp3.x, sp2.y+i);
				//draw
				self.draw_line(x1 as u16, (sp2.y+i) as u16, x2 as u16, (sp2.y+i) as u16);
			}
		}
		else {panic!("Invalid style");}
	}


/*

	//removed color from vertex
	
	pub fn draw_shaded_triangle(&mut self, v1: &vertex, v2: &vertex, v3: &vertex){
		if let (Some(color1), Some(color2), Some(color3)) = (&v1.color, &v2.color, &v3.color){
			let mut p1 = v1.pos.to_vec2();
			let mut p2 = v2.pos.to_vec2();
			let mut p3 = v3.pos.to_vec2();
			let mut c1 = color1.copy();
			let mut c2 = color2.copy();
			let mut c3 = color3.copy();
			if p1.y > p2.y{
				vec2::swap(&mut p1, &mut p2);
				Color::swap(&mut c1, &mut c2);
			}
			if p1.y > p3.y{
				vec2::swap(&mut p1, &mut p3);
				Color::swap(&mut c1, &mut c3);
			}
			if p2.y > p3.y{
				vec2::swap(&mut p2, &mut p3);
				Color::swap(&mut c2, &mut c3);
			}
			if p1.y == p2.y && p1.x>p2.x{vec2::swap(&mut p1, &mut p2);}

			for i in 0..(p2.y-p1.y+1.0) as u16{
				let i = f32::from(i);
				let x1 = lerp(p1.y, p1.x, p2.y, p2.x, p1.y+i);
				let cx1 = Color::mix(&c1, &c2, i as f32/(p2.y-p1.y) as f32);
				let x2 = lerp(p1.y, p1.x, p3.y, p3.x, p1.y+i);
				let cx2 = Color::mix(&c1, &c3, i as f32/(p2.y-p1.y) as f32);
				self.draw_shaded_line(&vec2{x:x1, y:(p1.y+i)}, &vec2{x:x2, y:(p1.y+i)}, &cx1, &cx2);
			}
			for i in 0..(p3.y-p2.y+1.0) as u16{
				let i = f32::from(i);
				let x1 = lerp(p2.y, p2.x, p3.y, p3.x, p2.y+i);
				let cx1 = Color::mix(&c1, &c2, i as f32/(p3.y-p2.y) as f32);
				let x2 = lerp(p1.y, p1.x, p3.y, p3.x, p2.y+i);
				let cx2 = Color::mix(&c1, &c3, i as f32/(p3.y-p2.y) as f32);
				self.draw_shaded_line(&vec2{x:x1 , y:(p2.y+i)}, &vec2{x:x2, y:(p2.y+i+1.0)}, &cx1, &cx2);
			}
		}		
		else {
			self.draw_triangle("FILL", &v1.pos.to_vec2(), &v2.pos.to_vec2(), &v3.pos.to_vec2());
		}
	}

	//private on purpose
	fn draw_shaded_line(&mut self, p1:&vec2, p2:&vec2, c1:&Color, c2:&Color){
		
		let x1 = p1.x as u16;
		let x2 = p2.x as u16;
		let y1 = p1.y as u16;
		let y2 = p2.y as u16;

		let diffx = if x2>x1 {x2-x1+1} else {x1-x2+1};
		let diffy = if y2>y1 {y2-y1+1} else {y1-y2+1};

		if diffx > diffy{
			for i in 0..diffx{
				let x3 = if x2>x1 {x1+i} else {x1-i};
				let y3 = lerp(x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32) as u16;
				let c3 = Color::mix(c1, c2, i as f32/diffx as f32);
				self.put_pixel_with_color(x3, y3, c3);
			}
		}
		else{
			for i in 0..diffy{
				let y3 = if y2>y1 {y1+i} else {y1-i};
				let x3 = lerp(y1 as f32, x1 as f32, y2 as f32, x2 as f32, y3 as f32) as u16;
				let c3 = Color::mix(c1, c2, i as f32/diffy as f32);
				self.put_pixel_with_color(x3, y3, c3);
			}
		}
	}
*/



}