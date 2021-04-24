use super::Behavior;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use rand::prelude::*;
use rand::Rng;
use random_fast_rng::{local_rng, Random};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;
use std::{fs, io};
pub enum Error {
    NotFound,
}
// ## 验证码结构体
// * original_image
// * original_block
// * dst_image
// * dst_block

#[derive(Clone)]
pub struct Pair {
    pub dst_image: DynamicImage,
    pub dst_block: DynamicImage,
    pub dst_image_base64: String,
    pub dst_block_base64: String,
    pub x: u32,
    pub y: u32,
}
pub struct DstDoubleBuffer {
    pub dst_buffer_one: Vec<Pair>,
    pub dst_buffer_two: Vec<Pair>,
    // pub rng: Arc<Mutex<ThreadRng>>
}
pub type DoubleBuffer = Arc<Mutex<DstDoubleBuffer>>;
impl DstDoubleBuffer {
    pub fn new() -> DoubleBuffer {
        Arc::new(Mutex::new(DstDoubleBuffer {
            dst_buffer_one: Vec::new(),
            dst_buffer_two: Vec::new() // rng: Arc::new(Mutex::new(thread_rng()))
            
        }))
    }
}
pub struct Captcha {
    // original image read from images/jigsaw/original/*.png
    original_image: Vec<DynamicImage>,
    // original block read from  images/jigsaw/slidingBlock/*.png
    original_block: Vec<DynamicImage>,
    // after generate will push to this field
    dst_double_buffer: DoubleBuffer,
    // next_time to gennerate, timestamp
    // 10 bits
    next_epoch: u64,
    rng: ThreadRng,
}
pub fn get_captcha(dst_double_buffer: DoubleBuffer) -> Result<Pair, Error> {
    let dst = dst_double_buffer.lock().unwrap();
    if dst.dst_buffer_one.len() > 0 {
        let random_u8 = local_rng().get_usize();
        let n = random_u8 % dst.dst_buffer_one.len();
        let pair = dst.dst_buffer_one[n].clone();
        Ok(pair)
    } else {
        Err(Error::NotFound)
    }
}
impl Captcha {
    pub fn new(dst_double_buffer: DoubleBuffer) -> Self {
        Captcha {
            original_block: Vec::new(),
            original_image: Vec::new(),
            dst_double_buffer: dst_double_buffer,
            next_epoch: 0,
            rng: rand::thread_rng(),
        }
    }
    fn visit_dir(path: String) -> io::Result<Vec<PathBuf>> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        Ok(entries)
    }
    fn load_original(&mut self, file: PathBuf) {
        let original_image = image::open(file).unwrap().to_rgba8();
        self.original_image
            .push(DynamicImage::ImageRgba8(original_image));
    }
    fn load_block(&mut self, file: PathBuf) {
        let original_block = image::open(file).unwrap();
        self.original_block.push(original_block);
    }
    fn rand_origin_block(&mut self) -> (DynamicImage, DynamicImage) {
        let rand1 = self.rng.gen_range(0, self.original_block.len());
        let rand2 = self.rng.gen_range(0, self.original_image.len());
        // 抠图图片
        let slidingblock = self.original_block[rand1].clone();
        // 原生图片
        let original = self.original_image[rand2].clone();
        (slidingblock, original)
    }
    fn generate(&mut self) -> bool {
        let (mut slidingblock, mut original) = self.rand_origin_block();
        // 固定抠图x的坐标
        let rand_x = self
            .rng
            .gen_range(10, original.width() - 2 * slidingblock.width() - 20);
        // y的坐标固定为0，方便滑动的时候只进行往右滑动
        let rand_y = 0;
        for x in 0..slidingblock.width() {
            for y in 0..slidingblock.height() {
                // 找到不是透明的像素,把原生的像素copy到抠图图片
                let pixel = slidingblock.get_pixel(x, y);
                if pixel[3] > 0 {
                    // 获取原生图片当前像素值
                    let org_pixel = original.get_pixel(x + rand_x, y);
                    // 把原生像素拷贝到抠图位置上
                    slidingblock.put_pixel(x, y, org_pixel);
                    // 原生图像增加阴影
                    original.put_pixel(
                        x + rand_x,
                        y + rand_y,
                        image::Rgba([org_pixel[0], org_pixel[1], org_pixel[2], 200]),
                    );
                    // 在右边加入干扰图
                    let other_pixel = original.get_pixel(x + rand_x + slidingblock.width() + 10, y);
                    // 干扰图增加阴影
                    original.put_pixel(
                        x + rand_x + slidingblock.width(),
                        y + rand_y,
                        image::Rgba([other_pixel[0], other_pixel[1], other_pixel[2], 200]),
                    );
                }
            }
        }
         use std::io::Cursor;
        let (mut dst_block_buff, mut dst_image_buff) =
                (Cursor::new(vec![]), Cursor::new(vec![]));
           slidingblock 
                .write_to(&mut dst_block_buff, image::ImageOutputFormat::Png)
                .unwrap();

            let dst_block_base64 = base64::encode(dst_block_buff.get_ref());
           original 
                .write_to(&mut dst_image_buff, image::ImageOutputFormat::Png)
                .unwrap();
            let dst_image_base64 = base64::encode(dst_image_buff.get_ref());

        // 随机存放在dst_pair_list
        let p = Pair {
            dst_block: slidingblock,
            dst_image: original,
            x: rand_x,
            y: rand_y,
            dst_block_base64: dst_block_base64,
            dst_image_base64: dst_image_base64 
        };
        if self.dst_double_buffer.lock().unwrap().dst_buffer_one.len() < 10 {
            self.dst_double_buffer
                .lock()
                .unwrap()
                .dst_buffer_one
                .push(p);
        } else {
            let n = self.rng.gen_range(
                0,
                self.dst_double_buffer.lock().unwrap().dst_buffer_one.len(),
            );
            self.dst_double_buffer.lock().unwrap().dst_buffer_one[n] = p;
        }
        true
    }
}
impl Behavior for Captcha {
    fn init(&mut self) -> bool {
        // load all original image to buffer
        match Captcha::visit_dir(String::from("images/jigsaw/original/")) {
            Ok(entries) => {
                println!("{:?}", entries);
                for file in entries {
                    self.load_original(file)
                }
            }
            Err(e) => println!("Load image: {:?}", e.to_string()),
        }
        match Captcha::visit_dir(String::from("images/jigsaw/slidingBlock/")) {
            Ok(entries) => {
                println!("{:?}", entries);
                for file in entries {
                    self.load_block(file)
                }
            }
            Err(e) => println!("Load block: {:?}", e.to_string()),
        }
        true
    }
    fn name(&self) -> &str {
        "captcha"
    }
    fn run(&mut self) -> bool {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                if n.as_secs() > self.next_epoch {
                    if self.dst_double_buffer.lock().unwrap().dst_buffer_one.len() > 0{
                        self.generate();
                    }else{
                        for _ in 0..9{
                            self.generate();
                        }
                    }
                    // reset next epoch
                    self.next_epoch = n.as_secs() + 60;
                }
            }
            Err(_) => {}
        }
        true
    }
    fn terminate(&mut self) -> bool {
        println!("captcha terminate");
        true
    }
}
