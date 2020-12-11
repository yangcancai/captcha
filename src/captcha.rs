use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;
use image::{DynamicImage};
use std::{fs,io};
use image::GenericImageView;
use image::GenericImage;
use super::lib::Behavior;
use std::path::PathBuf;
use rand::Rng;
use rand::prelude::*;
pub enum Error{
        NotFound
    }
// ## 验证码结构体
// * original_image
// * original_block
// * dst_image
// * dst_block

pub struct Pair{
     dst_image: DynamicImage,
     dst_block: DynamicImage,
     x: u32,
     y: u32
} 
pub struct Captcha{
    // original image read from images/jigsaw/original/*.png
    original_image: Vec<DynamicImage>,
    // original block read from  images/jigsaw/slidingBlock/*.png
    original_block: Vec<DynamicImage>,
    // after generate will push to this field
    dst_pair_list: Vec<Pair>,
    // next_time to gennerate, timestamp
    // 10 bits
    next_epoch: u64,
    rng: ThreadRng
}
impl Captcha {
    pub fn new() -> Self{
        Captcha{
            original_block: Vec::new(),
            original_image: Vec::new(),
            dst_pair_list:  Vec::new(),
            next_epoch: 0,
            rng: rand::thread_rng()
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
    fn generate(&mut self) -> bool{
        // 固定抠图x的坐标
     let rand_x = 80;
     // y的坐标固定为0，方便滑动的时候只进行往右滑动
     let rand_y = 0;
    // 抠图图片
    let mut slidingblock= self.original_block[0].clone();
    // 原生图片
    let mut original=  self.original_image[0].clone();
        for x in 0..slidingblock.width(){
        for y in 0..slidingblock.height(){
        // 找到不是透明的像素,把原生的像素copy到抠图图片
        let pixel = slidingblock.get_pixel(x, y);
        if pixel[3] > 0 {
            // 获取原生图片当前像素值 
            let org_pixel = original.get_pixel(x + rand_x, y);
            // 把原生像素拷贝到抠图位置上
            slidingblock.put_pixel(x, y, org_pixel);
            // 原生图像增加阴影
            original.put_pixel(x + rand_x,  y + rand_y, image::Rgba([org_pixel[0], org_pixel[1], org_pixel[2], 200]));
            // 在右边加入干扰图
             let other_pixel = original.get_pixel(x+rand_x + slidingblock.width() + 10, y);
            // 干扰图增加阴影
             original.put_pixel(x + rand_x + slidingblock.width(), y + rand_y, image::Rgba([other_pixel[0], other_pixel[1], other_pixel[2], 200]));
        }
        }
    }
    // 随机存放在dst_pair_list
    let p = Pair{
            dst_block: slidingblock,
            dst_image: original,
            x: rand_x,
            y: rand_y 
        };
    if self.dst_pair_list.len() < 10 {
        self.dst_pair_list.push(p);
    }else{
        let n = self.rng.gen_range(0, self.dst_pair_list.len());
        self.dst_pair_list[n] = p;
    }
    true
    }
    
    // 获取结果
    pub fn get_captcha(&mut self) -> Result<&Pair, Error>{
        if self.dst_pair_list.len() > 0 {
        let n = self.rng.gen_range(0, self.dst_pair_list.len());
        Ok(&self.dst_pair_list[n])
    }else{
        Err(Error::NotFound) 
    }
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
    fn run(& mut self) -> bool{
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                if n.as_secs() > self.next_epoch {
                    self.generate();
                    // reset next epoch
                    self.next_epoch = n.as_secs() + 60;
                }
            },
            Err(_) =>{

            }
        }
        true
    }
    fn terminate(&mut self) -> bool{
        println!("captcha terminate");
        true
    }
    
}