// src/html_to_webp.rs
use std::process::Command;
use std::fs::File;
use std::io::Write;

pub fn generate_webp_from_html(html_content: &str, output_image_path: &str) {
    // Write the HTML content to a temporary file
    let html_file_path = "temp.html";
    let mut html_file = File::create(html_file_path).expect("Failed to create HTML file");
    html_file.write_all(html_content.as_bytes()).expect("Failed to write HTML content");

    // Use wkhtmltoimage to convert the HTML to WebP
    let status = Command::new("wkhtmltoimage")
        .arg("--format")
        .arg("webp")
        .arg(html_file_path)
        .arg(output_image_path)
        .status()
        .expect("Failed to execute wkhtmltoimage");

    // Check if the command was successful
    if status.success() {
        println!("Successfully generated WebP image at {}", output_image_path);
    } else {
        eprintln!("Failed to generate WebP image");
    }

    // Clean up the temporary HTML file
    std::fs::remove_file(html_file_path).expect("Failed to delete temporary HTML file");
}
