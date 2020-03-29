fn main() {
    prost_build::compile_protos(&["src/stat.proto"],
                                &["src/"]).unwrap();
}
