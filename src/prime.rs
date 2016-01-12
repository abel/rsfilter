use std::f64;

pub fn is_prime(candidate: u32)->bool {
    if (candidate & 1) == 0{
        return candidate == 2
    }
    let num = f64::sqrt(candidate as f64) as u32;
    let mut i = 3;
    while i <= num{
        if candidate % i == 0{
            return false
        }
        i+=2;
    }
    true
}

// known prime numbers
const PRIMES:&'static [u32] = &[3, 7, 11, 0x11, 0x17, 0x1d, 0x25, 0x2f, 0x3b, 0x47, 0x59, 0x6b, 0x83,
 0xa3, 0xc5, 0xef,0x125, 0x161, 0x1af, 0x209, 0x277, 0x2f9, 0x397, 0x44f, 0x52f, 0x63d, 0x78b, 0x91d,
 0xaf1, 0xd2b, 0xfd1, 0x12fd, 0x16cf, 0x1b65, 0x20e3, 0x2777, 0x2f6f, 0x38ff, 0x446f, 0x521f, 0x628d,
 0x7655, 0x8e01, 0xaa6b, 0xcc89, 0xf583, 0x126a7, 0x1619b,0x1a857, 0x1fd3b, 0x26315, 0x2dd67, 0x3701b,
 0x42023, 0x4f361, 0x5f0ed, 0x72125, 0x88e31, 0xa443b, 0xc51eb, 0xec8c1, 0x11bdbf, 0x154a3f, 0x198c4f,
 0x1ea867, 0x24ca19, 0x2c25c1, 0x34fa1b, 0x3f928f, 0x4c4987, 0x5b8b6f, 0x6dda89];

pub fn get_prime(min:u32)->u32{
    //在已知数据中查找
    for num2 in PRIMES.iter(){
         if *num2 >= min{
            return *num2
         }
    }
    //通过计算获得
    let mut next = min;
    if (next & 1) == 0{
        next+=1;
    }
    while next < 0x7fffffff{
        if is_prime(next){
            return next
        }
        next+=2
    }
    0
}

pub fn get_hash_code(text: &[u8])->u32{
    let count = text.len();
    let mut h:u32 = count as u32;
    for i in 0..count{
        h = (h << 5) - h + text[i] as u32;
    }
    h & 0x7fffffff
}

pub fn text_equals(s1: &[u8], s2: &[u8]) ->bool{
    let count = s1.len();
    if count == s2.len(){
        for i in 0..count{
            if s1[i] != s2[i]{
                return false
            }
        }
        return true
    }
    false
}
