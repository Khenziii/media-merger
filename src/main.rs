mod files;

fn main() {
    files::validate_env("./input", "./output");
    let files = files::get_files("./input");
}
