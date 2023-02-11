mod util;
mod window_handler;

use util::*;
use window_handler::Window_Handler;

use minifb::Key;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

	let mut window_handler = Window_Handler::new(WIDTH, HEIGHT);

    while window_handler.window.is_open() && !window_handler.window.is_key_down(Key::Escape) {
    	
    	window_handler.set_color(Color(0, 255, 0));

    	drawCube(&mut window_handler);

    	window_handler.render();
	
	}
}


fn drawCube(window_handler:&mut Window_Handler){
	const d:f32 = 250.0;//z' dist from camera to viewport assuming camera is at 0

    //cube!!
	let points = Vec::from([
		vec3{x:270.0, y:130.0, z:100.0}, //0
		vec3{x:370.0, y:130.0, z:100.0}, //1
		vec3{x:370.0, y:230.0, z:100.0}, //2
		vec3{x:270.0, y:230.0, z:100.0}, //3
		vec3{x:270.0, y:130.0, z:200.0}, //4
		vec3{x:370.0, y:130.0, z:200.0}, //5
		vec3{x:370.0, y:230.0, z:200.0}, //6
		vec3{x:270.0, y:230.0, z:200.0}  //7
		]);

	let index_buffer = Vec::from([
		0, 1, 2,
		1, 2, 3,

		0, 4, 5,
		4, 5, 1,
		
		0, 4, 7,
		4, 7, 3,

		1, 5, 6,
		5, 6, 2,

		4, 5, 6,
		5, 6, 7,

		2, 3, 7,
		3, 7, 6
		]);
		
		//for points in polygon
	for i in (0..12){

		//let normal = vec3::cross(points[index_buffer[i*3]]-points[index_buffer[(i*3)+1]]);

		let p1 = vec2{
			x: (d*points[index_buffer[i*3]].x)/(d+points[index_buffer[i*3]].z),
			y: (d*points[index_buffer[i*3]].y)/(d+points[index_buffer[i*3]].z)
		};
		let p2 = vec2{
			x: (d*points[index_buffer[(i*3)+1]].x)/(d+points[index_buffer[(i*3)+1]].z),
			y: (d*points[index_buffer[(i*3)+1]].y)/(d+points[index_buffer[(i*3)+1]].z)
		};
		let p3 = vec2{
			x: (d*points[index_buffer[(i*3)+2]].x)/(d+points[index_buffer[(i*3)+2]].z),
			y: (d*points[index_buffer[(i*3)+2]].y)/(d+points[index_buffer[(i*3)+2]].z)
		};
		window_handler.draw_triangle("WIREFRAME", &p1, &p2, &p3);
		//window_handler.draw_triangle("FILL", &p1, &p2, &p3);


	}
}