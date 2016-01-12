pub mod prime;
pub mod trie;
/*
#[test]
fn test_prime() {
    let isprime1 = prime::is_prime(589);
    let isprime2 = prime::is_prime(79);
    let prime = prime::get_prime(999);
    assert_eq!(isprime1,false);
    assert_eq!(isprime2,true);
    assert_eq!(1103, prime);
    println!("prime:{}", prime);
}

#[test]
fn test_hashcode() {
    let code1 = prime::get_hash_code("589".as_bytes());
    let code2 = prime::get_hash_code("9999".as_bytes());
    assert_eq!(142099, code1);
    assert_eq!(5448772, code2);
    println!("code1:{},code2:{}", code1, code2);
}

#[test]
fn test_text_equals() {
    let code1 = prime::text_equals("589".as_bytes(),"9999".as_bytes() );
    let code2 = prime::text_equals("9999".as_bytes(),"9999".as_bytes() );
    assert_eq!(code1, false);
    assert_eq!(code2, true);
    println!("code1:{},code2:{}", code1, code2);
}
*/

#[test]
fn test_trie_filter_find_first() {
     let mut t = trie::TrieFilter::new(true);
     t.add_ignore_chars("@#");
     t.add_keyword("天朝");
     t.add_keyword("fuck");
     t.add_keyword("小日本");
     {
         let op_bad_word = t.find_first("BBbddFuCkadsafds");
         match op_bad_word{
             None=>(),
             Some(bad_word)=>{
                 assert_eq!(bad_word, "FuCk");
             }
         }
    }
    {
        let op_bad_word = t.find_first("BBcka小日本dsafds");
        match op_bad_word{
            None=>(),
            Some(bad_word)=>{
                assert_eq!(bad_word, "小日本");
            }
        }
   }
}

#[test]
fn test_trie_filter_find_all() {
     let mut t = trie::TrieFilter::new(true);
     t.add_ignore_chars("@#");
     t.add_keyword("天朝");
     t.add_keyword("fuck");
     t.add_keyword("小日本");
     {
        let all_word = t.find_all("BBbdItfuckerates  clones and then appendsdFuCkadsa小日本fds");
        assert_eq!(all_word, ["fuck","FuCk","小日本"]);
    }
    {
       let all_word = t.find_all("BBbdItfuckerates clonesand then appendsdFuCkadsa小@日#本fds");
       assert_eq!(all_word, ["fuck","FuCk","小@日#本"]);
   }
}

#[test]
fn test_trie_filter_replace() {
     let mut t = trie::TrieFilter::new(true);
     t.add_ignore_chars("@#");
     t.add_keyword("天朝");
     t.add_keyword("fuck");
     t.add_keyword("小日本");
     {
        let word = t.replace("BBbdItfuckerates  clones and then appendsdFuCkadsa小日本fds", "*".as_bytes()[0]);
        assert_eq!(word, "BBbdIt*erates  clones and then appendsd*adsa*fds");
    }
    {
       let word = t.replace("BBbdItfuckerates clonesand then appendsdFuCkadsa小@日#本fds", "*".as_bytes()[0]);
       assert_eq!(word, "BBbdIt*erates clonesand then appendsd*adsa*fds");
   }
}
