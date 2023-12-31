extern crate image;

use std::env;
use std::error::Error;

use image::{GenericImageView, Pixel};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::image::ImageGenerationRequest;
use reqwest;

const OPENAI_API_KEY_VAR_NAME: &str = "OPENAI_API_KEY";

fn fetch_image(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    Ok(bytes.to_vec())
}

fn image_to_ascii(image_data: &[u8]) -> Result<String, image::ImageError> {
    let img = image::load_from_memory(image_data)?;

    let img = img.resize_exact(80, 40, image::imageops::FilterType::Nearest);

    let ascii_chars = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

    let mut ascii_art = String::new();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).to_luma();
            let brightness = pixel[0] as f32 / 255.0;
            let char_idx = (brightness * (ascii_chars.len() - 1) as f32) as usize;
            ascii_art.push(ascii_chars.chars().nth(char_idx).unwrap());
        }
        ascii_art.push('\n');
    }

    Ok(ascii_art)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut prompt = String::new();

    if args.len() > 1 {
        prompt = args[1].clone();
        println!("Generating image for prompt: {}", prompt);
    } else {
        println!("No argument provided");
    }

    if env::var(OPENAI_API_KEY_VAR_NAME).is_err() {
        println!("Unable to find OpenAI API Key. Set the environment variable {} and try again.", OPENAI_API_KEY_VAR_NAME);
    }

    let client = Client::new(env::var(OPENAI_API_KEY_VAR_NAME).unwrap().to_string());
    let request = ImageGenerationRequest::new(prompt);
    let result = client.image_generation(request)?;
    let url = &result.data[0].url;

    let image_bytes = fetch_image(url)?;

    let ascii_converted = image_to_ascii(&image_bytes)?;

    let split = ascii_converted.split("\n");

    for str in split {
        println!("{}", str);
    }
    Ok(())
}