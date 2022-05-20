fn main() {
    build_proto();
    tauri_build::build();
}

fn build_proto() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
