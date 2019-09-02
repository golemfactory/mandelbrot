use std::fs::File;
use std::fs;
use std::io::{BufWriter};
use std::path::{Path};

use failure::{Error, Fail};


#[derive(Debug, Fail)]
pub enum SaveFileError {
    #[fail(display = "Can't save file. Buffer size doesn't match expected width {} and height {}.", width, height)]
    NotMatchingSize {
        width: u32,
        height: u32,
    },
    #[fail(display = "Can't find parent")]
    NoParent,
}



pub fn save_file(output: &str, data: &Vec<u8>, width: u32, height: u32) -> Result<(), Error> {

    if data.len() != (width * height) as usize {
        return Err(SaveFileError::NotMatchingSize{ width, height })?;
    }

    let path = Path::new(output);
    fs::create_dir_all(path.parent().ok_or(SaveFileError::NoParent)?)?;

    let file = File::create(&path)?;

    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(data)?;
    Ok(())
}

pub fn load_file(input: &Path) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(input).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    return buf
}



