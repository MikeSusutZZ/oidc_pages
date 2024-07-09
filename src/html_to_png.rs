use std::process::Command;
use std::fs::File;
use std::io::Write;

/// Generates a PNG image from HTML content.
///
/// # Arguments
///
/// * `html_content` - A string slice that holds the HTML content to be converted.
/// * `output_image_path` - A string slice that holds the path where the generated PNG image will be saved.
/// Example call:
/// html_to_webp::generate_png_from_html(html_content, output_image_path);

pub fn generate_png_from_html(html_content: &str, output_image_path: &str) {
    // Write the HTML content to a temporary file
    let html_file_path = "temp.html";
    let mut html_file = File::create(html_file_path).expect("Failed to create HTML file");
    html_file.write_all(html_content.as_bytes()).expect("Failed to write HTML content");

    // Use wkhtmltoimage to convert the HTML to PNG
    let output = Command::new("wkhtmltoimage")
        .arg("--format")
        .arg("png")
        .arg(html_file_path)
        .arg(output_image_path)
        .output()
        .expect("Failed to execute wkhtmltoimage");

    // Debugging information
    println!("wkhtmltoimage stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("wkhtmltoimage stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Check if the command was successful
    if output.status.success() {
        println!("Successfully generated PNG image at {}", output_image_path);
    } else {
        eprintln!("Failed to generate PNG image with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Clean up the temporary HTML file
    std::fs::remove_file(html_file_path).expect("Failed to delete temporary HTML file");
}
