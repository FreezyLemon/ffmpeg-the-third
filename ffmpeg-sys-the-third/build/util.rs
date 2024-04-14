use std::env;
use std::path::PathBuf;

pub fn cargo_feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

pub fn ffmpeg_version() -> String {
    env!("CARGO_PKG_VERSION")
        .split('+')
        .nth(1)
        .unwrap()
        .replace("ffmpeg-", "")
}

pub fn ffmpeg_major_version() -> u32 {
    ffmpeg_version().split('.').next().unwrap().parse().unwrap()
}

pub fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn source() -> PathBuf {
    output().join(format!("ffmpeg-{}", ffmpeg_version()))
}

pub fn search() -> PathBuf {
    let mut absolute = env::current_dir().unwrap();
    absolute.push(&output());
    absolute.push("dist");

    absolute
}
