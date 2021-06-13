extern crate image;
extern crate hound;

fn main() {
	println!("WAV file: ");
	let audio_filename = user_input();
	println!("Output: ");
	let output_filename = user_input() + ".png";

	let mut audio_file = hound::WavReader::open(audio_filename).unwrap();
	let spec = audio_file.spec();

	if spec.sample_format != hound::SampleFormat::Int || spec.bits_per_sample != 16 {
		println!("Unsupported audio file!");
		return;
	}
	let raw_samples = audio_file.samples::<i16>().into_iter().map(|x| x.unwrap()).collect::<Vec<i16>>();

	let mut samples: Vec<i16> = Vec::new();

	for i in 0..=raw_samples.len() - 1 {
		if i % 100 == 0 {
			samples.push(raw_samples[i as usize]);
		}
	}

	let mut img = image::RgbImage::new(samples.len() as u32, 655);

	println!("Generating image...");

	for x in 0..=(samples.len()-1) as u32 {
		let sample_x = (((samples[x as usize] as i32) + 32767) as f32 /100.0) as u32;
		for y in 0..=(655-1) as u32 {
			if sample_x == y {
				img.put_pixel(x, y, image::Rgb([115, 115, 255]));
			} else if y > sample_x {
				img.put_pixel(x, y, image::Rgb([115, 115, 255]));
			} else if y == 32766 {
				img.put_pixel(x, y, image::Rgb([127, 127, 127]));
			} else {
				img.put_pixel(x, y, image::Rgb([100, 100, 100]));
			}
		}
    }

    println!("Writing to file...");
    img.save(output_filename).unwrap();
    println!("Done!");
}

fn user_input() -> String { //get input from user
    use std::io::stdin;
    let mut str = String::new();
    stdin().read_line(&mut str).expect("Invalid input");
    str = str.replace("\r\n", "");
    return str;
}