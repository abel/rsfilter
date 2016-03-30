#[macro_use]
extern crate lazy_static;
use std::ptr;

pub struct Point {
    x: i32,
    y: i32,
}
static mut defaultPoint: *const Point = 0 as *const Point;

lazy_static! {
    pub static ref POINT:Point = {
        let m = Point{x:888,y:999};
        m
    };
}

lazy_static! {
    pub static ref NUMBER: u32 = times_two(3);
}

fn times_two(n: u32) -> u32 {
    n * 2
}

fn point_two(p: &mut Point){
    p.x = p.x * 2;
    p.y = p.y * 2;
}

#[test]
fn test_point_two() {
    let mut p = Point{x:888,y:999};
    point_two(&mut p);
    assert_eq!(p.x, 888*2);
}

#[test]
fn test_lazy_static() {
    let init_point = Box::into_raw(Box::new(Point{x:888,y:999}));
    unsafe {
        let mut p: *const Point = 0 as *const Point;
        p = init_point;
        println!("x:{}, y:{}", (*p).x,(*p).y);
        assert_eq!((*p).x, 888);
        assert_eq!((*p).y, 999);
    }
    assert_eq!(*NUMBER, 6);
    assert_eq!((*POINT).x, 888);
    assert_eq!((*POINT).y, 999);
}


#[test]
fn test_default_point() {
    let mut init_point = Box::into_raw(Box::new(Point{x:888,y:999}));
    unsafe {
        //let mut defaultPoint: *mut Point = 0 as *mut Point;
        defaultPoint = init_point;
        println!("x:{}, y:{}", (*defaultPoint).x,(*defaultPoint).y);
        assert_eq!((*defaultPoint).x, 888);
        assert_eq!((*defaultPoint).y, 999);
    }
}
