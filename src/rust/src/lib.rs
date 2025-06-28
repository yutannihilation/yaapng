// Example functions

use std::fs::File;
use std::io::{self, Write};

use savvy::savvy;

use savvy::StringSexp;

#[savvy]
fn apng(
    png_files: StringSexp,
    apng_file: &str,
    delay_num: f64,
    delay_den: f64,
) -> savvy::Result<()> {
    // Create in-memory buffer for APNG
    let mut png_buffer = Vec::new();

    // Convert delay from seconds to milliseconds
    let delay_num_ms = (delay_num * 1000.0) as u16;
    let delay_den_ms = (delay_den * 1000.0) as u16;

    // Collect all frames
    let mut frames = Vec::new();
    let mut actual_width = 0u32;
    let mut actual_height = 0u32;
    let mut palette = None;

    for (i, p) in png_files.iter().enumerate() {
        let reader = io::BufReader::new(File::open(p)?);
        let decoder = png::Decoder::new(reader);

        let mut png_reader = decoder.read_info()?;
        let info = png_reader.info();

        if info.is_animated() {
            return Err(savvy::savvy_err!(
                "Input file must be non-animated PNG: {p}"
            ));
        }

        let (frame_width, frame_height) = info.size();

        // For the first frame, set the dimensions
        if i == 0 {
            actual_width = frame_width;
            actual_height = frame_height;

            // Store palette if it's an indexed image
            if info.color_type == png::ColorType::Indexed {
                palette = info.palette.clone();
            }
        } else {
            // Check if subsequent frames have the same dimensions
            if frame_width != actual_width || frame_height != actual_height {
                return Err(savvy::savvy_err!(
                    "Frame dimensions ({frame_width}x{frame_height}) do not match first frame dimensions ({actual_width}x{actual_height})"
                ));
            }
        }

        let color_type = info.color_type;
        let bit_depth = info.bit_depth;

        // Get the actual output buffer info
        let output_info = png_reader.output_buffer_size();
        let mut image_buf = vec![0; output_info];

        png_reader.next_frame(&mut image_buf)?;

        frames.push((image_buf, color_type, bit_depth));
    }

    if frames.is_empty() {
        return Err(savvy::savvy_err!("No frames to process"));
    }

    // Create PNG encoder for APNG using actual frame dimensions
    let mut encoder = png::Encoder::new(&mut png_buffer, actual_width, actual_height);

    // Use the color type and bit depth from the first frame
    let (_, first_color_type, first_bit_depth) = &frames[0];

    encoder.set_color(*first_color_type);
    encoder.set_depth(*first_bit_depth);

    // Set palette if we have one (for indexed images)
    if let Some(ref pal) = palette {
        encoder.set_palette(pal.clone());
    }

    // Set up animation control
    encoder.set_animated(frames.len() as u32, 0)?;
    encoder.set_frame_delay(delay_num_ms, delay_den_ms)?;
    encoder.set_compression(png::Compression::Best);

    let mut writer = encoder.write_header()?;

    // Write all frames
    for (i, (frame_data, _, _)) in frames.iter().enumerate() {
        if i > 0 {
            // For subsequent frames, we need to set frame control
            writer.set_frame_delay(delay_num_ms, delay_den_ms)?;
        }
        writer.write_image_data(frame_data)?;
    }

    writer.finish()?;

    // Optimize the APNG using oxipng
    let opts = oxipng::Options::default();
    let optimized = oxipng::optimize_from_memory(&png_buffer, &opts)
        .map_err(|e| savvy::savvy_err!("Failed to optimize APNG: {e}"))?;

    // Write optimized APNG to file
    let mut output_file = File::create(apng_file)?;
    output_file.write_all(&optimized)?;

    Ok(())
}
