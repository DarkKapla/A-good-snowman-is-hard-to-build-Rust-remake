//! Read the map.txt file and extract the dimension (width, height)
//! of the map so that the two of them are constants in the code.
fn main() {
	println!("cargo:rerun-if-changed=map.txt");

	const MAP: &[u8] = include_bytes!("map.txt");

	let size_x = MAP.iter().filter(|&&b| b == b'\n').count();
	let size_y = MAP
		.iter()
		.enumerate()
		.find_map(|(i, &b)| if b == b'\n' { Some(i) } else { None })
		.unwrap_or_else(|| MAP.len());

	assert!(size_x | size_y != 0, "The map is empty!");
	assert!(
		size_x < 1000 && size_y < 1000,
		"The map is too big, height and width must not exceed 3 digits."
	);

	println!("cargo:rustc-env=SNOWCRAB_SIZE_X={:03}", size_x);
	println!("cargo:rustc-env=SNOWCRAB_SIZE_Y={:03}", size_y);
}
