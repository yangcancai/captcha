 use image::GenericImageView;
 use image::GenericImage;
 use image::RgbImage;
 use image::Rgb;
mod lib;
 use lib::{point};
 use lib::manager;
 fn main() {
     manager::run();
     // 固定抠图x的坐标
     let rand_x = 80;
     // y的坐标固定为0，方便滑动的时候只进行往右滑动
     let rand_y = 0;
    // 抠图图片
    let mut slidingblock= image::open("images/jigsaw/slidingBlock/1.png").unwrap();
    // 原生图片
    let mut original= image::open("images/jigsaw/original/bg1.png").unwrap().to_rgba();
    for x in 0..slidingblock.width(){
        for y in 0..slidingblock.height(){
        // 找到不是透明的像素,把原生的像素copy到抠图图片
        let pixel = slidingblock.get_pixel(x, y);
        if pixel[3] > 0 {
            // 获取原生图片当前像素值 
            let org_pixel = *original.get_pixel(x + rand_x, y);
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
    // 写入文件
    slidingblock.save("sliding_1.png").unwrap();
    original.save("org_bg1.png").unwrap();
 }