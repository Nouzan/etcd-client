fn main() {
    let proto_root = "proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    if std::env::var_os("CARGO_FEATURE_V3_5").is_some() {
        let proto_root = "proto/v3.5";
        tonic_build::configure()
            .build_server(false)
            .compile(
                &[
                    "proto/v3.5/auth.proto",
                    "proto/v3.5/kv.proto",
                    "proto/v3.5/rpc.proto",
                    "proto/v3.5/v3election.proto",
                    "proto/v3.5/v3lock.proto",
                ],
                &[proto_root],
            )
            .expect("Failed to compile proto files");
    } else {
        tonic_build::configure()
            .build_server(false)
            .compile(
                &[
                    "proto/auth.proto",
                    "proto/kv.proto",
                    "proto/rpc.proto",
                    "proto/v3election.proto",
                    "proto/v3lock.proto",
                ],
                &[proto_root],
            )
            .expect("Failed to compile proto files");
    }
}
