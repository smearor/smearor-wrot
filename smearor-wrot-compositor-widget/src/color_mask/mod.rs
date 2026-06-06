// /// Convert RGB values (0.0-1.0) to HEX color string (e.g., "#2C2C2C")
// pub fn rgb_to_hex(r: f32, g: f32, b: f32) -> String {
//     let r = (r * 255.0).clamp(0.0, 255.0) as u8;
//     let g = (g * 255.0).clamp(0.0, 255.0) as u8;
//     let b = (b * 255.0).clamp(0.0, 255.0) as u8;
//     format!("#{:02X}{:02X}{:02X}", r, g, b)
// }

// /// Apply color mask to pixel data
// /// Replaces pixels matching the mask color with transparency (chroma-keying)
// pub fn apply_color_mask(pixel_data: &mut Vec<u8>, mask_color: (f32, f32, f32, f32)) {
//     let (mask_r, mask_g, mask_b, tolerance) = mask_color;
//     let tolerance_sq = tolerance * tolerance;
//
//     for i in (0..pixel_data.len()).step_by(4) {
//         let b = pixel_data[i] as f32 / 255.0;
//         let g = pixel_data[i + 1] as f32 / 255.0;
//         let r = pixel_data[i + 2] as f32 / 255.0;
//         // Alpha is at i + 3
//
//         let dr = r - mask_r;
//         let dg = g - mask_g;
//         let db = b - mask_b;
//
//         let distance_sq = dr * dr + dg * dg + db * db;
//
//         if distance_sq <= tolerance_sq {
//             pixel_data[i + 3] = 0; // Set alpha to 0 (transparent)
//         }
//     }
// }

pub mod color_mask_applier;
