use image::{GenericImageView, Rgba, Rgb};

extern crate image;

fn main() {
    let msg = "隐藏文字";
    let s = String::from(msg);
    for c in msg.chars() {
        println!("{}", c as u32)
    }
    // byte2Bin(msg);
    let path = "input1.jpg";
    let input = image::open(path).unwrap();
    let (width, height) = input.dimensions();
    let mut output: image::RgbaImage = image::ImageBuffer::new(width, height);
    // encode_msg(&input.pixels(), msg);
    // println!("{:?}", input.pixels());
    // for mut i in input.pixels() {
    //     i.2 = Rgba([i.2[0], i.2[1], i.2[3], i.2[4]]);
    //     println!("{:?}", i.2);
    // }
    encode_msg(&mut input.pixels().collect(), msg);
    input.save(path.replace(".", "-out.")).unwrap();
    // output.save(path.replace(".", "-out.")).unwrap();
}

fn encode_msg(colors: &mut Vec<(u32, u32, Rgba<u8>)>, msg: &str) {
    let msgBits = vec![get_bit_from_num(msg.len() as u32), get_msg_bit(msg)].concat() ;
    let mut history = vec![];
    let mut pos = 0;
    while pos < msgBits.len() {
        let mut loc = get_next_location(&mut history, colors.len() as u32);
        // colors[0];
        let mut  target = &colors[loc as usize].2;
        while (loc + 1) % 4 != 0 {
            loc = loc + 1;
        }
        colors[loc as usize] = 255;
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
        if (loc >= total as i32) {
            loc = 0;
        } 
        else if history.binary_search(&(loc as u32)).is_ok() {
            loc = loc + 1;
        } else {
            history.push(loc as u32);
            return loc;
        }
    }
}

fn byte2Bin(b: &[u8]) {
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
