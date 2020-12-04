 use image::GenericImageView;
 use image::GenericImage;
 use image::RgbImage;
 use image::Rgb;
 mod lib;
 use crate::lib::{Task, Context, Behavior, Director, Actor, Property, Plug, Man};
 mod captcha;
 use crate::captcha::Captcha;
 /// 管理模块
 pub mod manager{
     use super::*;
     pub fn run() {
         /// plug必须为单一的stuct，编译时判断运行时消耗小，代码不够灵活
        let mut context = Context::new();
        let plug: Plug = Task::new();
        context.init();
       context.install(&Plug{name: "plug02.."});
       context.install(&plug);
        // context.install(&timer);
        context.run();
        context.terminate();

        // 代码比较灵活的写法
        let mut director = Director::new();
        let mut actor = Actor::new();
        actor.set_property("name".to_string(), Property::STR("mayun".to_string()));
        actor.set_property("age".to_string(), Property::INT(32));
        actor.set_property("money".to_string(), Property::FLOAT(56666.00));

        let mut player: Actor = Actor::new();
        player.set_property( "name".to_string(), Property::STR("yy".to_string()));
        player.set_property("level".to_string(), Property::INT(12));
        let mut man = Man::new("solo".to_string());
        man.prop = player; 
       let cap = Captcha::new();
        director.install(Box::new(cap));
        director.install(Box::new(actor));
        director.install(Box::new(man));
        director.init();
        director.run();
        director.terminate();
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