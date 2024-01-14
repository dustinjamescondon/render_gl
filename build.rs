use std::{path::{PathBuf, Path}, env, fs};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("manifest_dir:{}", manifest_dir_string);
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).parent().unwrap().join("target").join(build_type);
    return PathBuf::from(path);
}

/// This is just for testing the font loading
fn main() {
// ...
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = get_output_path();
    let src = Path::join(&env::current_dir().unwrap(), "res/FreeSansBold.ttf");
    println!("{}", src.to_str().unwrap());
    let dest = Path::join(Path::new(&target_dir), Path::new("res/FreeSansBold.ttf"));
    println!("{}", dest.to_str().unwrap());
    fs::copy(src, dest).unwrap();
// ...
}