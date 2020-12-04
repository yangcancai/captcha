use std::vec::Vec;
use image::{DynamicImage, ImageDecoderExt};
use image::ImageBuffer;
use image::Rgba;
use std::{fs,io};
use std::io::Error;
use super::lib::Behavior;
use std::path::PathBuf;
// ## 验证码结构体
// * original_image
// * original_block
// * dst_image
// * dst_block

pub struct Pair{
     dst_image: DynamicImage,
     dst_block: DynamicImage
} 
pub struct Captcha{
    // original image read from images/jigsaw/original/*.png
    original_image: Vec<DynamicImage>,
    // original block read from  images/jigsaw/slidingBlock/*.png
    original_block: Vec<DynamicImage>,
    pair: Pair
}

impl Captcha {
    pub fn new() -> Self{
        Captcha{
            original_block: Vec::new(),
            original_image: Vec::new(),
            pair: Pair{
                dst_image: DynamicImage::new_rgba8(0, 0), 
                dst_block: DynamicImage::new_rgb8(0, 0)}
        }
    }
    fn visit_dir(path: String) -> io::Result<Vec<PathBuf>> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        Ok(entries)
    }
    fn load_original(& mut self, file: PathBuf) {
        let original_image = image::open(file).unwrap().to_rgba();
        self.original_image.push(DynamicImage::ImageRgba8(original_image));
    }
    fn load_block(& mut self, file: PathBuf) {
        let original_block = image::open(file).unwrap();
        self.original_block.push(original_block);
    }
}
impl Behavior for Captcha{
    fn init(& mut self) -> bool{
        // load all original image to buffer
        match Captcha::visit_dir(String::from("images/jigsaw/original/")){
           Ok(entries) => {
               println!("{:?}",entries);
               for file in entries{
                     self.load_original(file)
               }
        },
           Err(e) => 
               println!("Load image: {:?}", e.to_string())
        }
       match Captcha::visit_dir(String::from("images/jigsaw/slidingBlock/")){
           Ok(entries) => {
               println!("{:?}",entries);
               for file in entries{
                     self.load_block(file)
               }
        },
           Err(e) => 
               println!("Load block: {:?}", e.to_string())
        }
       true
    }
    fn name(&self) -> &str{
        "captcha"
    }
    
}