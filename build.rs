use std::fs::DirEntry;
use std::{env, fs, path::PathBuf, process::Command};
//-----------------------------------------------------------------------------

fn compile_glsl_shader(file: &DirEntry, mut out_pathbuf: PathBuf) {
    // get filename
    let file_name = file.file_name().into_string().unwrap();

    // create <filename>.spv extension
    let mut file_name_path = PathBuf::from(file_name);

    let mut file_name_ext = file_name_path
        .extension()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    file_name_ext.push_str(".spv");

    file_name_path.set_extension(file_name_ext);

    // push <filename>.spv to out folder
    out_pathbuf.push(file_name_path);

    let infile = file.path().into_os_string().into_string().unwrap();
    let outfile = out_pathbuf.into_os_string().into_string().unwrap();

    Command::new("glslangValidator")
        .arg("-V")
        .arg(&infile)
        .arg("-o")
        .arg(&outfile)
        .output()
        .expect(&format!("Failed to compile shader {} !", &infile));
}
//-----------------------------------------------------------------------------

fn compile_shaders() {
    println!("build - Compiling shaders...");

    let shaders_pathbuf: PathBuf = [env!("CARGO_MANIFEST_DIR"), "resources", "shaders"]
        .iter()
        .collect();

    let mut src_pathbuf = shaders_pathbuf.clone();
    src_pathbuf.push("src");

    let mut dist_pathbuf = shaders_pathbuf.clone();
    dist_pathbuf.push("dist");

    let shader_entries = fs::read_dir(src_pathbuf).expect("Failed to read shaders src folder!");
    let shaders = shader_entries
        .into_iter()
        .map(|entry| entry.unwrap())
        .collect::<Vec<DirEntry>>();

    for shader in shaders {
        compile_glsl_shader(&shader, dist_pathbuf.clone());
    }

    println!("build - Shader compilation successful.")
}
//-----------------------------------------------------------------------------

fn main() {
    compile_shaders();
}
//-----------------------------------------------------------------------------
