use prime;

pub struct TrieSlot {
    end: bool,
    key: u8,
    next: u32,
    value: TrieNode,
}

pub struct TrieNode {
    buckets: Vec<u32>,
    slots: Vec<TrieSlot>,
}

impl TrieSlot{
    pub fn new(key:u8, next:u32)-> TrieSlot{
       TrieSlot{key:key, next:next, value:TrieNode::new(), end:false}
    }
}

impl TrieNode{
    pub fn new()-> TrieNode{
       TrieNode{buckets: Vec::new(), slots:Vec::new()}
    }

    fn increase_capacity(&mut self, capacity:usize) -> usize{
        let size = self.slots.len();
        let prime = prime::get_prime(capacity as u32) as usize;
        self.buckets.resize(prime, 0);
        //重新设置m_buckets和next
        for i in 0..size{
            self.buckets[i] = 0;
        }
        for i in 0..size {
            let index = (self.slots[i].key as usize ) % prime;
            self.slots[i].next = self.buckets[index];
            self.buckets[index] = (i+1) as u32;
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
        return 0
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

    pub fn add_key(&mut self, keys:&[u8], trans: &[u8;CHARCOUNT]){
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
        }
        self.slots[i].value.add_key(&keys[1..key_len], trans)
    }

    pub fn exists_key(&self, keys:&[u8], trans: &[u8;CHARCOUNT], depth: &mut i32) ->bool{
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
                self.exists_key(&keys[1..key_len], trans, depth)
            }else{
                false
            }
        }else{
            if self.slots[i-1].end{
                true
            }else{
                self.slots[i-1].value.exists_key(&keys[1..key_len], trans, depth)
            }
        }
    }
}

const CHARCOUNT : usize = 256;

pub struct TrieFilter  {
	transition: [u8;CHARCOUNT],
	root_node:  TrieNode,
}

fn get_non_zero(a:u8, b:u8) ->usize {
	if a != 0 {
		return a as usize
	}
	return b as usize
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
        //	AddReplaceChars(zh_TW, zh_CN);
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
    pub fn add_key(&mut self, key : &str){
        self.root_node.add_key(key.as_bytes(), &self.transition)
    }

    // 查找关键字的位置
    fn find_badword_index(&self, text:&[u8]) ->i32 {
        let mut depth = 0i32;
        if self.root_node.exists_key(text, &self.transition, &mut depth){
            depth
        }else{
            -1
        }
    }

    // 存在过滤字
    pub fn has_badword(&self, text :&str) ->bool {
        let bin = text.as_bytes();
        let len = bin.len();
    	for i in 0..len {
    		let index = self.find_badword_index(&bin[i..len]);
    		if index > 0 {
    			return true
    		}
    	}
    	return false
    }

    //查找第1个脏字
    pub fn find_first(&self, text :&str) -> Option<String> {
        let bin = text.as_bytes();
        let len = bin.len();
    	for i in 0..len  {
    		let index = self.find_badword_index(&bin[i..len]);
    		if index > 0 {
                let end_index = i+index as usize;
    			return Option::Some(String::from_utf8(bin[i..end_index].to_vec()).unwrap())
    		}
    	}
    	return Option::None
    }

    pub fn find_all(&self, text :&str) -> Vec<String> {
	    let mut all_badword : Vec<String> = Vec::new();
        let bin = text.as_bytes();
        let len = bin.len();
        let mut i = 0;
       	while i < len  {
       		let index = self.find_badword_index(&bin[i..len]);
       		if index > 0 {
                let end_index = i+index as usize;
       			all_badword.push(String::from_utf8(bin[i..end_index].to_vec()).unwrap());
                i += index as usize;
       		}else{
                i+=1;
            }
       	}
	    all_badword
    }

    pub fn replace(&self, text :&str, mask:u8) -> String {
        let bin = text.as_bytes();
        let len = bin.len();
    	let mut outbuffer: Vec<u8> = Vec::new();
    	let mut find_count = 0;
        let mut i = 0;
    	while i < len {
    	    let	index = self.find_badword_index(&bin[i..len]);
    		if index > 0 {
    			if find_count == 0 {
                    //outbuffer.push_all(&bin[0..i]);
                    for t in 0..i{
        			   outbuffer.push(bin[t]);
                    }
    			}
    			find_count+=1;
    			outbuffer.push(mask);
    			i += index as usize;
    		} else {
    			if find_count > 0 {
    			    outbuffer.push(bin[i]);
    			}
                i +=1
    		}
    	}
    	if find_count == 0 {
    		text.to_string()
    	}else{
            String::from_utf8(outbuffer).unwrap()
        }
    }

}
