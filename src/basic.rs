fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

fn fun() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn let_fun() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    let mut _d: i32 = 32;
    println!("(a + b) + (c + d) = {}", e);
}

fn pattern_fun() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b)
}

struct Struct {
    e: i32,
}

fn pattern_fun2() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5, 6];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 5, 5], [a, b, c, d, e])
}

const PAGE_SIZE: i32 = 100_000;

fn shadow_let_fn() {
    let x = 4;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner, {}", x);
    }
    println!("The value of x in outer, {}", x);
}

fn count_fn() {
    let space = "     ";
    println!("space len is {}", space.len())
}

fn num_fn() {
    let cur_num = 0.1 + 0.2;
    assert!((cur_num - 3_f64).abs() - 3_f64 < 0.000001);
    println!("{}", cur_num);
}

fn nan_fn() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("current num is  {}", "NaN")
    }
}

fn bit_fn() {
    let a: i32 = 2;
    let b: i32 = 3;

    println!("a & b is {}", a & b);
    println!("a | b is {}", a | b);
    println!("a ^ b is {}", a ^ b);
    println!("!b is {}", !b);
}

fn range_fn() {
    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}

fn font_fn() {
    let c = 'z';
    let c2 = "z";
    let c3 = '中';
    let c4 = "z中";
    println!(
        "内存占用 {}, {}, {}, {}",
        std::mem::size_of_val(&c),
        std::mem::size_of_val(&c2),
        std::mem::size_of_val(&c3),
        std::mem::size_of_val(&c4),
    )
}

fn bool_fn () {
    let t = true;
    if t {
        println!("{}", t)
    }
}

fn panic_fn () -> ! {
    panic!("崩溃")
}

fn loop_fn () -> ! {
    loop {
        
    }
}

fn main() {
    greet_world();
    fun();
    let_fun();
    pattern_fun();
    pattern_fun2();
    shadow_let_fn();
    count_fn();
    num_fn();
    nan_fn();
    bit_fn();
    range_fn();
    font_fn();
    bool_fn();
}
