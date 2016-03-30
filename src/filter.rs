
use trie;
use std::ptr;

//static defaultPoint: *const Point = ptr::null();
//static defaultPoint: *const Point = 0 as *const Property;

pub static mut defaultWordFilter: *const trie::TrieFilter = 0 as *const trie::TrieFilter;
pub static mut defaultNameFilter: *const trie::TrieFilter = 0 as *const trie::TrieFilter;

pub fn load_maskword_file(path: &str){
     let mut filter = Box::into_raw( Box::new(trie::TrieFilter::new(true)));
     unsafe {
         //let mut defaultPoint: *mut Point = 0 as *mut Point;
         (*filter).add_ignore_chars(" *&^%$#@!~,.:[]{}?+-~\"\\");
         (*filter).load_keyword_from_file(path);
         defaultWordFilter = filter;
     }
}

pub fn load_maskname_file(path: &str){
    let mut filter = Box::into_raw( Box::new(trie::TrieFilter::new(true)));
    unsafe {
        (*filter).load_keyword_from_file(path);
        defaultNameFilter = filter;
    }
}

pub fn creat_filter_from_maskword_file(path: &str) -> trie::TrieFilter{
    let mut filter = trie::TrieFilter::new(true);
    filter.add_ignore_chars(" *&^%$#@!~,.:[]{}?+-~\"\\");
    filter.load_keyword_from_file(path);
    filter
}

pub fn creat_filter_from_maskname_file(path: &str)-> trie::TrieFilter{
    let mut filter = trie::TrieFilter::new(true);
    filter.load_keyword_from_file(path);
    filter
}
