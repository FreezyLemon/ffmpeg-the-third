use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::util::*;
use crate::Library;

// left side: cargo feature name ("CARGO_FEATURE_BUILD_LIB_{}")
// right side: FFmpeg configure name ("--enable-{}")
static EXTERNAL_BUILD_LIBS: &[(&str, &str)] = &[
    // SSL
    ("GNUTLS", "gnutls"),
    ("OPENSSL", "openssl"),
    // Filters
    ("FONTCONFIG", "fontconfig"),
    ("FREI0R", "frei0r"),
    ("LADSPA", "ladspa"),
    ("ASS", "libass"),
    ("FREETYPE", "libfreetype"),
    ("FRIBIDI", "libfribidi"),
    ("OPENCV", "libopencv"),
    ("VMAF", "libvmaf"),
    // Encoders/decoders
    ("AACPLUS", "libaacplus"),
    ("CELT", "libcelt"),
    ("DCADEC", "libdcadec"),
    ("DAV1D", "libdav1d"),
    ("FAAC", "libfaac"),
    ("FDK_AAC", "libfdk-aac"),
    ("GSM", "libgsm"),
    ("ILBC", "libilbc"),
    ("VAZAAR", "libvazaar"),
    ("MP3LAME", "libmp3lame"),
    ("OPENCORE_AMRNB", "libopencore-amrnb"),
    ("OPENCORE_AMRWB", "libopencore-amrwb"),
    ("OPENH264", "libopenh264"),
    ("OPENH265", "libopenh265"),
    ("OPENJPEG", "libopenjpeg"),
    ("OPUS", "libopus"),
    ("SCHROEDINGER", "libschroedinger"),
    ("SHINE", "libshine"),
    ("SNAPPY", "libsnappy"),
    ("SPEEX", "libspeex"),
    ("STAGEFRIGHT_H264", "libstagefright-h264"),
    ("THEORA", "libtheora"),
    ("TWOLAME", "libtwolame"),
    ("UTVIDEO", "libutvideo"),
    ("VO_AACENC", "libvo-aacenc"),
    ("VO_AMRWBENC", "libvo-amrwbenc"),
    ("VORBIS", "libvorbis"),
    ("VPX", "libvpx"),
    ("WAVPACK", "libwavpack"),
    ("WEBP", "libwebp"),
    ("X264", "libx264"),
    ("X265", "libx265"),
    ("AVS", "libavs"),
    ("XVID", "libxvid"),
    // Protocols
    ("SMBCLIENT", "libsmbclient"),
    ("SSH", "libssh"),
];

trait FFmpegConfigure {
    fn switch(&mut self, feature: &str, option_name: &str);
    fn enable(&mut self, feature: &str, option_name: &str);
}

impl FFmpegConfigure for Command {
    fn switch(&mut self, feature: &str, option_name: &str) {
        let arg = if cargo_feature_enabled(feature) {
            format!("--enable-{option_name}")
        } else {
            format!("--disable-{option_name}")
        };

        self.arg(arg);
    }

    fn enable(&mut self, feature: &str, option_name: &str) {
        if cargo_feature_enabled(feature) {
            self.arg(format!("--enable-{option_name}"));
        }
    }
}

pub fn fetch(source_dir: &Path, ffmpeg_version: &str) -> io::Result<()> {
    let _ = std::fs::remove_dir_all(source_dir);
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg(format!("n{ffmpeg_version}"))
        .arg("https://github.com/FFmpeg/FFmpeg")
        .arg(source_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

pub fn build(
    out_dir: &Path,
    ffmpeg_version: &str,
    libraries: &[Library],
) -> io::Result<Vec<PathBuf>> {
    let install_dir = out_dir.join("dist");
    let source_dir = out_dir.join(format!("ffmpeg-{ffmpeg_version}"));
    let include_dir = install_dir.join("include");

    println!(
        "cargo:rustc-link-search=native={}",
        install_dir.join("lib").to_string_lossy()
    );

    if install_dir.join("lib").join("libavutil.a").exists() {
        // Already built
        rustc_link_extralibs(&source_dir);
        return Ok(vec![include_dir]);
    }

    fetch(&source_dir, ffmpeg_version)?;

    // Command's path is not relative to command's current_dir
    let configure_path = source_dir.join("configure");
    assert!(configure_path.exists());
    let mut configure = Command::new(&configure_path);
    configure.current_dir(&source_dir);

    configure.arg(format!("--prefix={}", install_dir.to_string_lossy()));

    if env::var("TARGET").unwrap() != env::var("HOST").unwrap() {
        // Rust targets are subtly different than naming scheme for compiler prefixes.
        // The cc crate has the messy logic of guessing a working prefix,
        // and this is a messy way of reusing that logic.
        let cc = cc::Build::new();
        let compiler = cc.get_compiler();
        let compiler = compiler.path().file_stem().unwrap().to_str().unwrap();
        let suffix_pos = compiler.rfind('-').unwrap(); // cut off "-gcc"
        let prefix = compiler[0..suffix_pos].trim_end_matches("-wr"); // "wr-c++" compiler

        configure.arg(format!("--cross-prefix={}-", prefix));
        configure.arg(format!(
            "--arch={}",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        ));
        configure.arg(format!(
            "--target_os={}",
            env::var("CARGO_CFG_TARGET_OS").unwrap()
        ));
    }

    // control debug build
    if env::var("DEBUG").is_ok() {
        configure.arg("--enable-debug");
        configure.arg("--disable-stripping");
    } else {
        configure.arg("--disable-debug");
        configure.arg("--enable-stripping");
    }

    // make it static
    configure.arg("--enable-static");
    configure.arg("--disable-shared");

    configure.arg("--enable-pic");

    // stop autodetected libraries enabling themselves, causing linking errors
    configure.arg("--disable-autodetect");

    // do not build programs since we don't need them
    configure.arg("--disable-programs");

    // the binary using ffmpeg-sys must comply with GPL
    configure.switch("BUILD_LICENSE_GPL", "gpl");

    // the binary using ffmpeg-sys must comply with (L)GPLv3
    configure.switch("BUILD_LICENSE_VERSION3", "version3");

    // the binary using ffmpeg-sys cannot be redistributed
    configure.switch("BUILD_LICENSE_NONFREE", "nonfree");

    let ffmpeg_major_version: u32 = ffmpeg_major_version();

    // configure building libraries based on features
    for lib in libraries
        .iter()
        .filter(|lib| lib.is_feature)
        .filter(|lib| !(lib.name == "avresample" && ffmpeg_major_version >= 5))
    {
        configure.switch(&lib.name.to_uppercase(), lib.name);
    }

    // configure external libraries based on features
    for (cargo_feat, option_name) in EXTERNAL_BUILD_LIBS {
        configure.enable(&format!("BUILD_LIB_{cargo_feat}"), option_name);
    }

    configure.enable("BUILD_DRM", "libdrm");
    configure.enable("BUILD_NVENC", "nvenc");
    // configure misc build options
    configure.enable("BUILD_PIC", "pic");

    // run ./configure
    let output = configure
        .output()
        .unwrap_or_else(|_| panic!("{:?} failed", configure));
    if !output.status.success() {
        println!("configure: {}", String::from_utf8_lossy(&output.stdout));

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "configure failed {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    let num_jobs = if let Ok(cpus) = std::thread::available_parallelism() {
        cpus.to_string()
    } else {
        "1".to_string()
    };

    // run make
    if !Command::new("make")
        .arg(format!("-j{num_jobs}"))
        .current_dir(&source_dir)
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    // run make install
    if !Command::new("make")
        .current_dir(&source_dir)
        .arg("install")
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
    }

    rustc_link_extralibs(&source_dir);

    Ok(vec![include_dir])
}

fn rustc_link_extralibs(source_dir: &Path) {
    let config_mak = source_dir.join("ffbuild/config.mak");
    let file = File::open(config_mak).unwrap();
    let reader = BufReader::new(file);
    let extra_libs = reader
        .lines()
        .find(|line| line.as_ref().unwrap().starts_with("EXTRALIBS"))
        .map(|line| line.unwrap())
        .unwrap();

    let linker_args = extra_libs.split('=').last().unwrap().split(' ');
    let include_libs = linker_args
        .filter(|v| v.starts_with("-l"))
        .map(|flag| &flag[2..]);

    for lib in include_libs {
        println!("cargo:rustc-link-lib={lib}");
    }
}
