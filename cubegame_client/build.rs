use fs_extra::dir::{copy, CopyOptions};
use std::{env, path::Path};

fn main() {
	// rerun this build script if assets has changed
	println!("cargo:rerun-if-changed=../assets/");

	let out_dir = env::var("OUT_DIR").unwrap();
	let target_dir = Path::new(&out_dir)
		.parent() // strip "out/" dir
		.unwrap()
		.parent() // strip hash dir
		.unwrap()
		.parent() // strip "build/"
		.unwrap();

	let cwd = env::current_dir().unwrap();
	let source = cwd.parent().unwrap().join("assets");

	// copying game assets into build output directory
	copy(source, target_dir, &CopyOptions::new().overwrite(true)).expect("Copying assets failed");
}
