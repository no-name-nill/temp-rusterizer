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

	const d:f32 = 250.0;//z' dist from camera to viewport assuming camera is at 0
	//const cam_pos: vec3 = vec3{x:0.0, y:0.0, z:0.0};
	const cam_dir: vec3 = vec3{x:1.0, y:1.0, z:1.0};

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
		2, 3, 0,

		0, 1, 5,
		5, 4, 0,

		0, 3, 7,
		7, 4, 0,

		1, 5, 6,
		6, 2, 1,

		4, 7, 6,
		6, 5, 4,

		2, 3, 7,
		7, 6, 2
		]);
		
	//for points in polygon
	for i in (0..12){

		let normal = vec3::cross(
			&(vec3::normalize(&(&points[index_buffer[(i*3)+0]]-&points[index_buffer[(i*3)+1]]))),
			&(vec3::normalize(&(&points[index_buffer[(i*3)+1]]-&points[index_buffer[(i*3)+2]])))
			);


		if(vec3::dot(&normal, &cam_dir)>0.0)
		{
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
			
			//window_handler.draw_triangle("WIREFRAME", &p1, &p2, &p3);
			window_handler.draw_triangle("FILL", &p1, &p2, &p3);

		}

	}
    	window_handler.render();
	
	}
}



