use protobuf_codegen::Customize;


fn main() -> Result<(), anyhow::Error> {
     protobuf_codegen::Codegen::new()
        .pure()
        .includes(["s2client-proto"])
        .inputs([
            "s2client-proto/s2clientprotocol/common.proto",
            "s2client-proto/s2clientprotocol/data.proto",
            "s2client-proto/s2clientprotocol/debug.proto",
            "s2client-proto/s2clientprotocol/error.proto",
            "s2client-proto/s2clientprotocol/query.proto",
            "s2client-proto/s2clientprotocol/raw.proto",
            "s2client-proto/s2clientprotocol/sc2api.proto",
            "s2client-proto/s2clientprotocol/score.proto",
            "s2client-proto/s2clientprotocol/spatial.proto",
            "s2client-proto/s2clientprotocol/ui.proto",
        ])
        .cargo_out_dir("proto")
        .customize(Customize::default().tokio_bytes(true))
        .run()
}
