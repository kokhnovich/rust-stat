extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::default();
    config.out_dir("src");

    config.compile_protos(&["src/stat.proto"], &["src/"]).unwrap();
}
