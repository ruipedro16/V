use std::fs;
use std::io::Write;

use reqwest::blocking::get;

pub fn create_file_verbose(file_path: &str, verbose: bool) {
    fs::write(file_path, "").expect("Failed to create test file");
    if verbose {
        log::info!("Created file: {}", file_path);
    }
}

pub fn copy_file_verbose(src: &str, dst: &str, verbose: bool) {
    fs::copy(src, dst).expect("Failed to copy file");
    if verbose {
        log::info!("Copied file: {} to {}", src, dst);
    }
}

pub fn download_file_verbose(url: &str, outpath: &str, verbose: bool) {
    let response = get(url).unwrap_or_else(|_| panic!("Failed to download file {}", url));

    if response.status().is_success() {
        let mut f_out = fs::File::create(outpath).expect("Failed to create file");
        let content = response.bytes().expect("Failed to get response bytes");
        f_out.write_all(&content).expect("Failed to write to file");

        if verbose {
            log::info!("Downloaded file: {} to {}", url, outpath);
        }
    } else {
        log::error!("Failed to download file {} to {}", url, outpath);
    }
}

pub fn create_dir_verbose(dir_path: &str, verbose: bool) {
    fs::create_dir_all(dir_path)
        .unwrap_or_else(|_| panic!("Failed to create directory {}", dir_path));

    if verbose {
        log::info!("Created directory: {}", dir_path);
    }
}
