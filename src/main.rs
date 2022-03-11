extern crate image;

use image::{GenericImageView, Rgba, Rgb};


fn main() {
    let msg = "隐藏文字";
    let message_bits = vec![get_bit_from_num(msg.chars().count() as u32), get_msg_bit(&msg)].concat();
    let path = "input1.jpg";
    let input = image::open(path).unwrap();
    let (width, height) = input.dimensions();
    let mut output: image::RgbaImage = image::ImageBuffer::new(width, height);
    let mut image_vec: Vec<u8> = vec![];
    // let mut output_image_vec = vec![];
    let mut num = 0;
    for (_, _, rgba) in input.pixels() {
        num += 1;
        image_vec = vec![image_vec, rgba.0.to_vec()].concat();
    }
    println!("{}", num);
    println!("{}", image_vec.len());
    // let mut history: Vec<u32> = vec![];
    // let mut pos = 0;
    // let msg_bit_len = message_bits.len();
    // while pos < msg_bit_len {
    //     let mut loc = get_next_location(&mut history, image_vec.len() as u32);
    //     image_vec[loc as usize] = set_bit(&(image_vec[loc as usize] as u8), &0, &(message_bits[pos as usize] as u8));
    //     while ((loc + 1) % 4) != 0 {
    //         loc+=1;
    //     }
    //     image_vec[loc as usize] = 255;
    //     pos+=1;
    // }
    // let mut a = vec![];
    // for i in 0..image_vec.len() {
    //     if (i % 4 == 0) && (i != 0) {
    //         a.push(vec![image_vec[i - 4], image_vec[i - 3], image_vec[i - 2], image_vec[i-1]]);
    //     }
    //     if i > 263998 * 4 {
    //         println!("{}", i)
    //     }
    // }
    // println!("{}", a.len());
    // let mut num = 0;
    // for (x, y, _) in input.pixels()  {
    //     // let b = &a[num];
    //     // output.put_pixel(x, y, Rgba([b[0], b[1], b[2], b[3]]));
    //     num+=1;
    // }
    // println!("{}", num);
    // for i in 0..image_vec.len() {
    //     let (x, y, rgba) = image_vec[i];
    //     if (i < messageBits.len()) {
    //         let a = set_bit(&rgba.0[2], &0, &(messageBits[i] as u8));
    //         output_image_vec.push((x, y, Rgba([rgba.0[0], rgba.0[1], a, rgba.0[3]])));
    //     } else {
    //         output_image_vec.push((x, y, rgba));
    //     }
    // }
    // image_vec[0..20].iter().for_each(|x| println!("{:?}", x));
    // println!("------------");
    // output_image_vec[0..20].iter().for_each(|x| println!("{:?}", x))
    // for index in 0..messageBits.len() {
    //     // let a = inputPixels;
    // }
    
    // byte_to_bin(msg.as_bytes());
    // let path = "input1.jpg";
    // let input = image::open(path).unwrap();
    // let (width, height) = input.dimensions();
    // let mut output: image::RgbaImage = image::ImageBuffer::new(width, height);
    // encode_msg(&input.pixels(), msg);
    // println!("{:?}", input.get_pixel(0, 0));
    // for (x, y, mut rgba) in input.pixels() {
    //     // i.2 = Rgba([i.2[0], i.2[1], i.2[3], i.2[4]]);
    //     output.put_pixel(x, y, Rgba([rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]]))
    // }
    // encode_msg(&mut input.pixels().collect(), msg);
    // input.save(path.replace(".", "-out.")).unwrap();
    // output.save(path.replace(".", "-out.")).unwrap();
}

fn encode_msg(colors: &mut Vec<(u32, u32, Rgba<u8>)>, msg: &str) {
    let msg_bits = vec![get_bit_from_num(msg.len() as u32), get_msg_bit(msg)].concat() ;
    let mut history = vec![];
    let mut pos = 0;
    while pos < msg_bits.len() {
        let mut loc = get_next_location(&mut history, colors.len() as u32);
        // colors[0];
        let mut  target_rgba = &colors[loc as usize].2;
        target_rgba = &Rgba([target_rgba[0], target_rgba[1], target_rgba[2], target_rgba[3]]);
        while (loc + 1) % 4 != 0 {
            loc = loc + 1;
        }
        // TODO
        // colors[loc as usize] = 255;
        pos = pos +1;
    }
}

fn get_msg_bit(msg: &str) -> Vec<u32> {
    let mut msg_bits = vec![];
    for i in msg.chars() {
        msg_bits.push(get_bit_from_num(i as u32));
    }
    msg_bits.concat()
}

fn get_bit_from_num(num: u32) -> Vec<u32> {
    let mut bits = vec![];
    for i in 0..16 {
        bits.push(get_bit(num, i))
    }
    bits
}

fn get_bit(num: u32, location: i32) -> u32 {
    (num >> location) & 1
}

fn get_next_location(history: &mut Vec<u32>, total: u32) -> i32 {
    let pos = history.len();
    let mut loc = ((pos + 1) as i32).abs() % total as i32;
    loop {
        if loc >= total as i32 {
            loc = 0;
        } 
        else if history.binary_search(&(loc as u32)).is_ok() {
            loc += 1;
        } else if ((loc + 1) % 4) == 0 {
            loc += 1;
        } else {
            history.push(loc as u32);
            return loc;
        }
    }
}

fn byte_to_bin(b: &[u8]) {
    for bit in b {
        println!("{}", bit);
    }
}

fn set_bit(num: &u8, location: &u8, bit: &u8) -> u8 {
    return (num & !(1 << location)) | (bit << location);
}

// fn to_hex(val: &str, len: usize) -> String {
//     let n: u32 = u32::from_str_radix(val, 8).unwrap();
//     format!("{:01$x}", n, len * 2)
// }
