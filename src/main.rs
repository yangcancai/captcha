 use image::GenericImageView;
 use image::GenericImage;
 use image::RgbImage;
 use image::Rgb;
mod lib;
 use lib::{point};
 use lib::manager;
 fn div(a: i32, b: i32) -> Option<i32>{
     if b == 0{
         None
     }else{
         Some(a/b)
     }
 }
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
    let mut xx: i32 = 99999; 
    for x in 0..slidingblock.width(){
        for y in 0..slidingblock.height(){
        // 找到不是透明的像素,把原生的像素copy到抠图图片
        let pixel = slidingblock.get_pixel(x, y);
        if pixel[3] > 0 {
            if (y as i32) < xx {
                xx = y as i32;
            }
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
    println!("xx = {}", xx);
    // 写入文件
    slidingblock.save("sliding_1.png").unwrap();
    original.save("org_bg1.png").unwrap();
 }
 fn test_draw(){
   let mut img2 = image::open("fractal.png").unwrap();
   let mut img1 = img2.to_rgba();
   let min_x = img1.width()/2-10;
   let min_y = img1.height()/4-10;
   let max_x = img1.width()-min_x;
   let max_y = img1.height()-min_y;
    // 画一个十字架
    for x in min_x..=max_x {
        for y in min_y..max_y {
            let pixel = *img1.get_pixel(x, y);
            img1.put_pixel(x, y, image::Rgba([pixel[0], pixel[1], pixel[2], 200]));
            let pixel = *img1.get_pixel(y, x);
            img1.put_pixel(y, x, image::Rgba([pixel[0], pixel[1], pixel[2], 200]));
        } 
   }
  let offx = img1.width() / 2;
  let offy = img1.height()/2;
  let r = 100;
  let mut x = 0;
  let mut y: i32 = 0;
  let mut xy = x * x + y * y;
  let  rr = r * r;
    // draw(& mut img1,  offx as i32, offy as i32, r);
    for x in x..r{
        for y in y..r{
            if x*x + y*y < rr{
             point(& mut img1, x , y, offx as i32, offy as i32);
             point(& mut img1, -x, -y, offx as i32, offy as i32);
             point(& mut img1, -x, y, offx as i32, offy as i32);
             point(& mut img1, x, -y, offx as i32, offy as i32);
            }
        }
    }
    img1.save("new_f.png").unwrap();


 }