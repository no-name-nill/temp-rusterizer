//Rusterizer: Rasterizer for Rustaceans

mod util;
mod window_handler;

use util::*;
use window_handler::WindowHandler;

use minifb::Key;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

	let mut window_handler = WindowHandler::new(WIDTH, HEIGHT);

    while window_handler.window.is_open() && !window_handler.window.is_key_down(Key::Escape) {
 	
    	//REFACTOR THIS, CLEAN THIS AND FIX ALL THIS

    	const d:f32 = 250.0;//z' dist from camera to viewport assuming camera is at 0
		let cam_pos = vec3{x:0.0, y:0.0, z:0.0};
		let light_source = vec3{x: 250.0, y: 50.0, z: 0.0};

    	//cube!!
		let points = Vec::from([

			vec3{x: 270.0, y: 130.0, z: 100.0}, //0
			vec3{x: 370.0, y: 130.0, z: 100.0}, //1
			vec3{x: 370.0, y: 230.0, z: 100.0}, //2
			vec3{x: 270.0, y: 230.0, z: 100.0}, //3
			vec3{x: 270.0, y: 130.0, z: 200.0}, //4
			vec3{x: 370.0, y: 130.0, z: 200.0}, //5
			vec3{x: 370.0, y: 230.0, z: 200.0}, //6
			vec3{x: 270.0, y: 230.0, z: 200.0}  //7
			
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
		for i in 0..12{

			let normal = vec3::cross(
				&(vec3::normalize(&(&points[index_buffer[(i*3)+1]]-&points[index_buffer[(i*3)+0]]))),
				&(vec3::normalize(&(&points[index_buffer[(i*3)+1]]-&points[index_buffer[(i*3)+2]])))
				);

			let cam_dir = vec3::normalize(&(&points[index_buffer[(i*3)+1]]-&cam_pos));
			if vec3::dot(&normal, &cam_dir)<0.0 //backface culling
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

				window_handler.putPixel(light_source.x as u16, light_source.y as u16);
				let light_dir1 = vec3::normalize(&(&light_source-&points[index_buffer[(i*3)+0]]));
				let light_dir2 = vec3::normalize(&(&light_source-&points[index_buffer[(i*3)+1]]));
				let light_dir3 = vec3::normalize(&(&light_source-&points[index_buffer[(i*3)+2]]));

				let light_intensity1 = (vec3::dot(&normal, &light_dir1)).max(0.0);
				let light_intensity2 = (vec3::dot(&normal, &light_dir2)).max(0.0); 
				let light_intensity3 = (vec3::dot(&normal, &light_dir3)).max(0.0);

				let v1 = vertex{pos: p1.to_vec3(),
					color: Some(Color(
						(255.0*light_intensity1)as u8,
						(255.0*light_intensity1)as u8,
						(255.0*light_intensity1)as u8,1.0))};
				let v2 = vertex{pos: p2.to_vec3(),
					color: Some(Color(
						(255.0*light_intensity2)as u8,
						(255.0*light_intensity2)as u8,
						(255.0*light_intensity2)as u8,1.0))};
				let v3 = vertex{pos: p3.to_vec3(),
					color: Some(Color(
						(255.0*light_intensity3)as u8,
						(255.0*light_intensity3)as u8,
						(255.0*light_intensity3)as u8,1.0))};
				window_handler.draw_shaded_triangle(&v1, &v2, &v3);

			}

		}
    	window_handler.render();
	}
}



