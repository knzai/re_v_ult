// fn process_cga_tile_bin(
//     path: &str,
//     canvas: &mut WindowCanvas,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let file = File::open(path)?;
//
//     let mut reader = BufReader::with_capacity(64, file);
//
//     //let mut y = 0;
//
//     loop {
//         let buffer = reader.fill_buf()?;
//
//         let buffer_length = buffer.len();
//
//         if buffer_length == 0 {
//             canvas.present();
//             break;
//         }
//         let mut x = 0;
//
//         for byte in buffer {
//             //
//             //print!("{}", x % 4 == 0);
//             if x % 4 == 0 {
//                 println!();
//             }
//             print!("{:08b}", byte);
//             x += 1;
//         }
//         // for byte in buffer {
//         //     let nib1: u8 = byte >> 4;
//         //     let nib2: u8 = byte & 0x0F;
//         //
//         //     //print!("{}", nib1);
//         //     //print!("{}", nib2);
//         //     if nib1 != 0 {
//         //         canvas.pixel(x, y, COLORS[nib1 as usize])?;
//         //     }
//         //     if nib2 != 0 {
//         //         canvas.pixel(x + 1, y, COLORS[nib1 as usize])?;
//         //     }
//         //     x += 2;
//         // }
//         //println!();
//
//         //y += 1;
//         reader.consume(buffer_length);
//     }
//
//     Ok(())
// }