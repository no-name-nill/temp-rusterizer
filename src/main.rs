//Rusterizer: Rasterizer for Rustaceans

mod util;
mod window_handler;
mod renderer;

use util::*;
use window_handler::WindowHandler;

use minifb::Key;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASPECT_RATIO: f32 = WIDTH as f32/HEIGHT as f32;

fn main() {

	let mut window_handler = WindowHandler::new(WIDTH, HEIGHT);

    while window_handler.window.is_open() && !window_handler.window.is_key_down(Key::Escape) {
    	//control the ups

    	let cube_model: Vec<util::vec4> = Vec::from([		
    		vec4{x: -1.0, y: -1.0, z:  -1.0, w: 1.0}, //0
			vec4{x:  1.0, y: -1.0, z:  -1.0, w: 1.0}, //1
			vec4{x:  1.0, y:  1.0, z:  -1.0, w: 1.0}, //2
			vec4{x: -1.0, y:  1.0, z:  -1.0, w: 1.0}, //3
			vec4{x: -1.0, y: -1.0, z:  1.0, w: 1.0}, //4
			vec4{x:  1.0, y: -1.0, z:  1.0, w: 1.0}, //5
			vec4{x:  1.0, y:  1.0, z:  1.0, w: 1.0}, //6
			vec4{x: -1.0, y:  1.0, z:  1.0, w: 1.0}  //7
		]);

		//use vertex here

		let obj_pos = vec3{x: 150.0, y: 100.0, z: 150.0};

		let scale = matrix4x4{data: [
			[50.0, 0.0,  0.0,  0.0],
			[0.0,  50.0, 0.0,  0.0],
			[0.0,  0.0,  50.0, 0.0],
			[0.0,  0.0,  0.0,  1.0]
			
			]};

		let translate = matrix4x4{data: [
			[1.0, 0.0, 0.0, obj_pos.x],
			[0.0, 1.0, 0.0, obj_pos.y],
			[0.0, 0.0, 1.0, obj_pos.z],
			[0.0, 0.0, 0.0, 1.0]
			]};

		let mut vertex_buffer = Vec::new();
		for i in (0..cube_model.len()){
			//scale -> translate ->project
			let scaled = scale.matrix_vec_mult(&cube_model[i]);
			vertex_buffer.push(translate.matrix_vec_mult(&scaled));
		}

		let index_buffer = Vec::from([

			0, 1, 2, //FRONT
			2, 3, 0, //FRONT
			0, 1, 5, //TOP
			5, 4, 0, //TOP
			6, 2, 3, //BOTTOM
			3, 7, 6, //BOTTOM
			1, 5, 6, //RIGHT
			6, 2, 1, //RIGHT
			4, 0, 3, //LEFT
			3, 7, 4, //LEFT
			4, 5, 6, //BACK
			6, 7, 4  //BACK
		
		]);

		const Z_NEAR: f32 = 0.5;
		const Z_FAR: f32 = 500.;
		let cam_pos: vec3 = vec3{x: 0.0, y: 0.0, z: 0.0};
		

    	let proj = matrix4x4{data: [
    		[Z_NEAR/ASPECT_RATIO, 0.0, 0.0, 0.0],
    		[0.0, Z_NEAR, 0.0, 0.0],
       		[0.0, 0.0, (Z_FAR+Z_NEAR), (Z_NEAR*Z_FAR*-1.)],
    		[0.0, 0.0, 1.0, 0.0]
    		]};

		//for points in polygon
		for i in 0..(index_buffer.len()/3){
			
			
			let normal = vec3::cross(
				&(vec3::normalize(&(&vertex_buffer[index_buffer[(i*3)+1]].to_vec3()
				-&vertex_buffer[index_buffer[(i*3)+2]].to_vec3()))),
				&(vec3::normalize(&(&vertex_buffer[index_buffer[(i*3)+1]].to_vec3()
				-&vertex_buffer[index_buffer[(i*3)+0]].to_vec3())))
				);


			let cam_dir = vec3::normalize(&(&vertex_buffer[index_buffer[(i*3)+1]].to_vec3()-&cam_pos));
			if (vec3::dot(&normal, &cam_dir))>0.0	//backface culling
			{
				//limit update per second

				//rotate!!

				let mut p1 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+0]]).to_vec3();
				let mut p2 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+1]]).to_vec3();
				let mut p3 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+2]]).to_vec3();

				p1.x = (p1.x+1.0)*(WIDTH/2) as f32;
				p1.y = (p1.y+1.0)*(HEIGHT/2) as f32;
				p1.z = ((p1.z*2.)/(Z_FAR-Z_NEAR))-1.;

				p2.x = (p2.x+1.0)*(WIDTH/2) as f32;
				p2.y = (p2.y+1.0)*(HEIGHT/2) as f32;
				p2.z = ((p2.z*2.)/(Z_FAR-Z_NEAR))-1.;
				
				p3.x = (p3.x+1.0)*(WIDTH/2) as f32;
				p3.y = (p3.y+1.0)*(HEIGHT/2) as f32;
				p3.z = ((p3.z*2.)/(Z_FAR-Z_NEAR))-1.;

				if i==0 || i==1{window_handler.set_color(Color(77, 78, 216, 1.0));}
				else if i==2 || i==3{window_handler.set_color(Color(234, 234, 232, 1.0));}
				else if i==4 || i==5{window_handler.set_color(Color(243, 196, 82, 1.0));}
				else if i==6 || i==7{window_handler.set_color(Color(81, 225, 84, 1.0));}
				else if i==8 || i==9{window_handler.set_color(Color(215, 78, 82, 1.0));}
				else if i==10 || i==11{window_handler.set_color(Color(213, 212, 76, 1.0));}
				window_handler.draw_triangle("FILL", &p1, &p2, &p3);
			}
		}
		window_handler.render();
	}
}



//eaeae8 rgb(234, 234, 232) //white
//4d4ed8 rgb(77, 78, 216)   //blue
//d74e52 rgb(215, 78, 82)	//red
//f3c452 rgb(243, 196, 82)  //orange
//51e154 rgb(81, 225, 84)   //green
//d5d44c rgb(213, 212, 76) //yellow