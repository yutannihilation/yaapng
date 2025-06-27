// Example functions

use std::fs::File;
use std::io::{self, BufWriter, Write};

use savvy::savvy;

use savvy::StringSexp;

#[savvy]
fn apng(png_files: StringSexp, apng_file: &str, delay: f64) -> savvy::Result<()> {
    // Create output file
    let output_file = File::create(apng_file)?;
    let mut writer = BufWriter::new(output_file);

    // Convert delay from seconds to milliseconds
    let delay_ms = (delay * 1000.0) as u16;

    // Collect all frames
    let mut frames = Vec::new();
    let mut actual_width = 0u32;
    let mut actual_height = 0u32;

    for (i, p) in png_files.iter().enumerate() {
        let reader = io::BufReader::new(File::open(p)?);
        let mut decoder = png::Decoder::new(reader);

        decoder.set_transformations(png::Transformations::EXPAND);

        let header_info = decoder.read_header_info()?;

        if header_info.is_animated() {
            return Err(savvy::savvy_err!(
                "Input file must be non-animated PNG: {p}"
            ));
        }

        let (frame_width, frame_height) = header_info.size();

        // For the first frame, set the dimensions
        if i == 0 {
            actual_width = frame_width;
            actual_height = frame_height;
        } else {
            // Check if subsequent frames have the same dimensions
            if frame_width != actual_width || frame_height != actual_height {
                return Err(savvy::savvy_err!(
                    "Frame dimensions ({frame_width}x{frame_height}) do not match first frame dimensions ({actual_width}x{actual_height})"
                ));
            }
        }

        let mut png_reader = decoder.read_info()?;

        // Get the actual output buffer info after transformations
        let output_info = png_reader.output_buffer_size();
        let mut image_buf = vec![0; output_info];

        png_reader.next_frame(&mut image_buf)?;

        // Store the transformed color type (after EXPAND transformation)
        let (output_color_type, output_bit_depth) = png_reader.output_color_type();

        frames.push((image_buf, output_color_type, output_bit_depth));
    }

    if frames.is_empty() {
        return Err(savvy::savvy_err!("No frames to process"));
    }

    // Create PNG encoder for APNG using actual frame dimensions
    let mut encoder = png::Encoder::new(&mut writer, actual_width, actual_height);

    // Use the color type and bit depth from the first frame
    let (_, first_color_type, first_bit_depth) = &frames[0];
    encoder.set_color(*first_color_type);
    encoder.set_depth(*first_bit_depth);

    // Set up animation control
    encoder.set_animated(frames.len() as u32, 0)?;
    encoder.set_frame_delay(delay_ms, delay_ms)?;

    let mut writer = encoder.write_header()?;

    // Write all frames
    for (i, (frame_data, _, _)) in frames.iter().enumerate() {
        if i > 0 {
            // For subsequent frames, we need to set frame control
            writer.set_frame_delay(delay_ms, delay_ms)?;
        }
        writer.write_image_data(frame_data)?;
    }

    writer.finish()?;

    Ok(())
}
