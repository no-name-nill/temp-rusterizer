use minifb::{Key, Window, WindowOptions};
use crate::util::*;



pub struct Window_Handler {
	pub window: Window,
	pub buffer: Vec<u32>,
	pub win_WIDTH: usize,
	pub win_HEIGHT: usize,
	pub current_color: u32

}

impl Window_Handler{
	pub fn new(WIDTH:usize, HEIGHT:usize)->Window_Handler{
		let mut window = Window::new(
			"Rusterizer!!",
			WIDTH,
			HEIGHT,
			WindowOptions::default()
			).unwrap();

		window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

		let mut buffer: Vec<u32> = vec![0;WIDTH*HEIGHT];

		Window_Handler{window: window,
			buffer: buffer,
			win_WIDTH: WIDTH,
			win_HEIGHT: HEIGHT,
			current_color: 0
		}
	}

	pub fn set_color(&mut self, color:Color){
		self.current_color = color.to_u32();
	}

	pub fn render(&mut self){
		self.window.update_with_buffer(&self.buffer, self.win_WIDTH, self.win_HEIGHT);
	}
	
	pub fn putPixel(&mut self, x:u16, y:u16){
		self.buffer[(x as u32 +y as u32 *self.win_WIDTH as u32)as usize] = self.current_color;
	}
	//pub fn putPoint(&mut self, p: vec2){}
	

	//pub fn draw_line(&mut self, p1: vec2, p2: vec2){
	pub fn draw_line(&mut self, x1:u16, y1:u16, x2:u16, y2:u16){
		let diffx = if x2>x1 {x2-x1} else {x1-x2};
		let diffy = if y2>y1 {y2-y1} else {y1-y2};

		if diffx > diffy{
			for i in (0..diffx){
				let x3 = if x2>x1 {x1+i} else {x1-i};
				let y3 = lerp(x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}
		else{
			for i in (0..diffy){
				let y3 = if y2>y1 {y1+i} else {y1-i};
				let x3 = lerp(y1 as f32, x1 as f32, y2 as f32, x2 as f32, y3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}

	}


	pub fn draw_triangle(&mut self, style:&str, p1: &vec2, p2: &vec2, p3: &vec2){
		//check if equal or collinear
		if (p2.y-p1.y)/(p2.x-p1.x)==(p3.y-p2.y)/(p3.x-p2.x){
			panic!("lines are collinear can't make a triangle");
		}

		if(style == "WIREFRAME"){
			self.draw_line(p1.x as u16, p1.y as u16, p2.x as u16, p2.y as u16);
			self.draw_line(p2.x as u16, p2.y as u16, p3.x as u16, p3.y as u16);
			self.draw_line(p1.x as u16, p1.y as u16, p3.x as u16, p3.y as u16);
		}
		else if(style == "FILL"){

			let mut sp1 = vec2{x:p1.x, y:p1.y};
			let mut sp2 = vec2{x:p2.x, y:p2.y};
			let mut sp3 = vec2{x:p3.x, y:p3.y};
			if sp1.y > sp2.y{vec2::swap(&mut sp1, &mut sp2);}
			if sp2.y > sp3.y{vec2::swap(&mut sp2, &mut sp3);}
			if sp1.y > sp3.y{vec2::swap(&mut sp1, &mut sp3);}

			if sp1.y == sp2.y && sp1.x>sp2.x{vec2::swap(&mut sp1, &mut sp2);}

			for i in 0..(sp3.y-sp1.y)as u16{
				let i = f32::from(i);
				//lerp line 1-2
				let x1 = lerp(sp1.y, sp1.x, sp2.y, sp2.x, sp1.y+i);
				//lerp line 1-3
				let x2 = lerp(sp1.y, sp1.x, sp3.y, sp3.x, sp1.y+i);
				//draw
				self.draw_line(x1 as u16, (sp1.y+i) as u16, x2 as u16, (sp1.y+i) as u16);
			}
		//shaded too
		}
		else {panic!("invalid style");}
	}


}