use std::str;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use prime;

pub struct TrieSlot {
    end: bool,
    key: u8,
    next: u32,
    value: TrieNode,
}

impl TrieSlot{
    pub fn new(key:u8, next:u32)-> TrieSlot{
       TrieSlot{key:key, next:next, value:TrieNode::new(), end:false}
    }
}

pub struct TrieNode {
    buckets: Vec<u32>,
    slots: Vec<TrieSlot>,
}

impl TrieNode{
    pub fn new()-> TrieNode{
       TrieNode{buckets: Vec::new(), slots:Vec::new()}
    }

    fn increase_capacity(&mut self, capacity:usize) -> usize{
        let prime = prime::get_prime(capacity as u32) as usize;
        //重新设置buckets和next
        for i in &mut self.buckets{
            *i = 0;
        }
        self.buckets.resize(prime, 0);
        let mut i = 0u32;
        for slot in &mut self.slots {
            let index = (slot.key as usize ) % prime;
            slot.next = self.buckets[index];
            i+=1;
            self.buckets[index] = i;
        }
        prime
    }

    //索引从1开始,0为没找到
    fn get_node_index(&self, key:u8) -> usize{
        let capacity = self.buckets.len();
        if capacity > 0{
            let size = self.slots.len();
            let index = (key as usize) % capacity;
            let mut i = self.buckets[index] as usize;
            while i > 0 && i <= size{
                i-=1;
                if self.slots[i].key == key{
                    return i + 1
                }
                i = self.slots[i].next as usize;
            }
        }
        0
    }

    fn add_char(&mut self, key:u8){
        //容量因子(1:1.5 = 67%)
        let size = self.slots.len();
        let mut capacity = self.buckets.len();
        let cp = size + (size >> 1);
        if capacity <= cp {
            capacity = self.increase_capacity(cp);
        }
        let index = key as usize % capacity;
        //如果有冲突,则指向前一个的位置,如果无冲突,则为无效索引0
        //(因为buckets中保存的位置从1开始.next保存的位置从1开始)
        let next = self.buckets[index];
        self.slots.push(TrieSlot::new(key, next));
        self.buckets[index] = 1 + size as u32;
    }

    pub fn add_keyword(&mut self, keys:&[u8], trans: &[u8;CHARCOUNT]){
        let key_len = keys.len();
        if key_len == 0{
            return
        }
        let mut key = keys[0];
        if key == 0{
            return
        }
        let tran = trans[key as usize];
        if tran != 0{
            key = tran;
        }
        let mut i = self.get_node_index(key);
        if i == 0{
            self.add_char(key);
            i = self.slots.len();
        }
        i-=1;
        if key_len == 1{
            self.slots[i].end = true;
            return
        }
        self.slots[i].value.add_keyword(&keys[1..key_len], trans)
    }

    pub fn exist_keyword(&self, keys:&[u8], trans: &[u8;CHARCOUNT], depth: &mut usize) ->bool{
        let key_len = keys.len();
        if key_len == 0{
            return false
        }
        *depth+=1;
        let mut key = keys[0];
        let tran = trans[key as usize];
        let ignore :bool;
        if tran == 0{
            ignore = true;
        }else{
            ignore = false;
            key = tran;
        }
        let i = self.get_node_index(key);
        if i == 0{
            if ignore{
                //被忽略的字符.跳过后继续找
                self.exist_keyword(&keys[1..key_len], trans, depth)
            }else{
                false
            }
        }else{
            if self.slots[i-1].end{
                true
            }else{
                self.slots[i-1].value.exist_keyword(&keys[1..key_len], trans, depth)
            }
        }
    }
}

const CHARCOUNT : usize = 256;
pub struct TrieFilter  {
    transition: [u8;CHARCOUNT],
    root_node:  TrieNode,
}

impl TrieFilter{
    pub fn new(ignore_case:bool)-> TrieFilter{
       let mut filter = TrieFilter{transition: [0;256], root_node:TrieNode::new()};
       for i in 0..CHARCOUNT{
           filter.transition[i] = i as u8;
       }
       filter.set_filter(ignore_case);
       filter
    }

    fn set_filter(&mut self, ignore_case:bool){
        //将小写转为大写字母
        if ignore_case {
            for a in 97..(97+26){
                self.transition[a as usize] = (a as u8) - 32; //(a:97,A:65);
            }
        }
        //简繁转换.暂未实现
        //if (ignore_simpTrad)
        //{
        //    AddReplaceChars(zh_TW, zh_CN);
        //}
    }

    //增加忽略字符
    pub fn add_ignore_chars(&mut self, pass_chars: &str) {
        for src in pass_chars.as_bytes(){
            self.transition[*src as usize] = 0;
        }
    }

    //增加替换字符
    pub fn add_replace_chars(&mut self, src_chars: &str, replace: &str) {
        let src_bin = src_chars.as_bytes();
        let replace_bin = replace.as_bytes();
        let mut count = src_bin.len();
        if count > replace_bin.len() {
            count = replace_bin.len();
        }
        for i in 0..count {
            self.transition[src_bin[i] as usize] = replace_bin[i];
        }
    }

    //添加关键字
    pub fn add_keyword(&mut self, text : &str){
        self.root_node.add_keyword(text.as_bytes(), &self.transition)
    }

    pub fn load_keyword_from_file(&mut self, path: &str) -> io::Result<()>{
        let f = try!(File::open(path));
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = try!(line);
            self.add_keyword(&line);
        }
        Ok(())
    }

    // 查找关键字的位置
    fn find_keyword_index(&self, text:&[u8]) ->usize {
        let mut depth = 0usize;
        if self.root_node.exist_keyword(text, &self.transition, &mut depth){
            depth
        }else{
            0
        }
    }

    // 是否存在关键字
    pub fn exist_keyword(&self, text :&str) ->bool {
        let bin = text.as_bytes();
        let len = bin.len();
        for i in 0..len {
            let index = self.find_keyword_index(&bin[i..len]);
            if index > 0 {
                return true
            }
        }
        false
    }

    //查找第1个关键字
    pub fn find_first<'a>(&self, text :&'a str) -> Option<&'a str> {
        let bin = text.as_bytes();
        let len = bin.len();
        for i in 0..len  {
            let index = self.find_keyword_index(&bin[i..len]);
            if index > 0 {
                return Some(str::from_utf8(&bin[i..(i+index)]).unwrap())
            }
        }
        Option::None
    }


    //查找所有关键字
    pub fn find_all<'a>(&self, text :&'a str) -> Vec<&'a str> {
        let mut all_keys: Vec<&'a str> = Vec::new();
        let bin = text.as_bytes();
        let len = bin.len();
        let mut i = 0;
        while i < len  {
            let index = self.find_keyword_index(&bin[i..len]);
            if index > 0 {
                all_keys.push(str::from_utf8(&bin[i..(i+index)]).unwrap());
                i += index as usize;
            }else{
                i+=1;
            }
        }
        all_keys
    }

    pub fn replace(&self, text :&str, mask:u8) -> String {
        let bin = text.as_bytes();
        let len = bin.len();
        let mut out_buffer: Vec<u8> = Vec::new();
        let mut find_count = 0;
        let mut i = 0;
        while i < len {
            let    index = self.find_keyword_index(&bin[i..len]);
            if index > 0 {
                if find_count == 0 {
                    //out_buffer.push_all(&bin[0..i]);
                    for t in 0..i{
                       out_buffer.push(bin[t]);
                    }
                }
                find_count += 1;
                out_buffer.push(mask);
                i += index as usize;
            } else {
                if find_count > 0 {
                    out_buffer.push(bin[i]);
                }
                i += 1;
            }
        }
        if find_count == 0 {
            text.to_string()
        }else{
            String::from_utf8(out_buffer).unwrap()
        }
    }

}
