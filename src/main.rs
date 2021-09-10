fn main1() {
    let s = "hello world".to_string();

    println!("addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}",
             &"hello world", &s, s.len(), s.capacity(), std::mem::size_of_val(&s));
}

static MAX: u32 = 0;

fn foo() {}
/*
RODATA: 0x10844381c
DATA (static var): 0x10844380c
TEXT (function): 0x10840e0f0
STACK (&hello): 0x7ffee77f40f0
HEAP (&*hello): 0x7fb0b5405a90
HEAP (box impl Pointer) 0x7fb0b5405aa0 0x7fb0b5405aa0
*/
fn main2() {
    let hello = "hello world".to_string();
    let data = Box::new(1);

    // string literals 指向 RODATA 地址
    println!("RODATA: {:p}", "hello world!");
    // static 变量在 DATA section
    println!("DATA (static var): {:p}", &MAX);
    // function 在 TEXT
    println!("TEXT (function): {:p}", foo as *const ());
    // String 结构体分配在栈上，所以其引用指向一个栈地址
    println!("STACK (&hello): {:p}", &hello);
    // 需要通过解引用获取其堆上数据，然后取其引用
    println!("HEAP (&*hello): {:p}", &*hello);
    // Box 实现了 Pointer trait 无需额外解引用
    println!("HEAP (box impl Pointer) {:p} {:p}", data, &*data);
}

fn main() {
    let a = "Hello";
    let b = "Tyr";

    let c = |msg: &str| {
        println!("{} {}: {}", a, b, msg);
    };

    c("How are you?");
}