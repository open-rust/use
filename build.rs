use macro_log::read_dir;

fn debug(obj: &impl std::fmt::Debug, tag: &str) {
    println!("cargo:warning=[Debug] {}: {tag} -> {obj:?}", file!());
}

fn main() {
    let mut cc = cc::Build::new();
    read_dir!("src/ffi").iter().for_each(|(file_name, _)| {
        if file_name.ends_with(".c") || file_name.ends_with(".cpp") {
            cc.file(format!("src/ffi/{file_name}"));
            debug(file_name, "compile file");
        }
    });
    cc.compile("ffi");
}
