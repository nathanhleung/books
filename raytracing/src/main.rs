fn main() {
	let image_width = 256;
	let image_height = 256;

	println!("P3");
	println!("{} {}", image_width, image_height);
	println!("255");
	for i in 0..image_width {
		eprintln!("Scanlines remaining: {}", image_width - i);
		for j in 0..image_height {
			let r = i as f64 / image_width as f64;
			let g = j as f64 / image_height as f64;
			let b = 0.25;

			let ir = (255.0 * r) as i64;
			let ig = (255.0 * g) as i64;	
			let ib = (255.0 * b) as i64;	
			
			println!("{} {} {}", ir, ig, ib);
		}	
	}	
	eprintln!("Done.");
}
