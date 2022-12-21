fn main() {
    println!("-------------------------------------------------------");
    // 숫자에 타입을 지정하지 않으면 i32 타입으로 추론됩니다.
    // 그러나 u8 타입으로 명시적으로 지정할 수 있습니다.
    let mynumber: u8 = 100; // 이경우 100은 u8타입이고 따라서 unicode 100에 해당하는 값은 'd'입니다.
    println!("The number is {}", mynumber as char);

    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());

    let slice2 = "안녕!";
    println!("Slice2 is {} bytes.", slice2.len());

    let small_number: u8 = 10;
    println!("Small number is {}", small_number);

    let small_number2 = 10u8;
    println!("small_number2 is {}", small_number2);
    println!("-------------------------------------------------------");
    println!("");
    println!("----------------------------type---------------------------");
    let first_letter = 'A';
    println!("first_letter is {}", first_letter);
    let space = ' ';
    println!("space is ||{}||", space);
    let other_languege_char = 'Ꮔ';
    println!("space is {}", other_languege_char);
    let cat_face = '😻';
    println!("space is {}", cat_face);
    println!("-------------------------------------------------------");
    println!("");
    println!("-------------------------------------------------------");
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
    println!("");
    println!("-------------------------------------------------------");

    let slice = "Hello!";
    println!(
        "Slice value is '{}' , {} bytes and also {} characters.",
        slice,
        slice.len(),
        slice.chars().count()
    );
    let slice2 = "안녕!";
    println!(
        "Slice2 value is '{}' , {} bytes and but {} characters.",
        slice2,
        slice2.len(),
        slice2.chars().count()
    );
    println!("---------------------------end of type----------------------------");
    println!("");
    println!("---------------------------Type interface----------------------------");
    let small_number1: u8 = 10; // basic
    println!("small_number1: {} ", small_number1);
    let small_number2 = 10u8; // 10 of type u8, and This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _

    println!("small_number2: {} ", small_number2);
    println!("big_number: {} ", big_number);

    // The _ does not change the number. It is only to make it easy for you to read. And it doesn't matter how many _ you use:
    let number = 0________u8;
    let number2 = 1___6______2____4______i32;
    println!("{}, {}", number, number2);

    // # Floats

    let my_float = 5.; // Rust sees . and knows that it is a float
    println!("my_float: {} ", my_float);

    let my_float: f64 = 5.0; // this is an f64
    println!("my_float: {} ", my_float);

    let my_other_float: f32 = 8.5; // this is an f32
    println!("my_other_float: {} ", my_other_float);

    let third_float = my_float + my_other_float as f64;
    // 타입 캐스팅 안해주면 에러남 ⚠️ mismatched types expected `f64`, found `f32`
    println!("third_float: {} ", third_float);

    let my_float = 5.0; // Rust will choose f64
    let my_other_float = 8.5; // Here again it will choose f64
    let third_float = my_float + my_other_float;
    println!("third_float: {} ", third_float);

    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // Usually Rust would choose f64,

    let third_float = my_float + my_other_float; // but now it knows that you need to add it to an f32. So it chooses f32 for my_other_float too
    println!("third_float: {} ", third_float);

    println!("---------------------------end of Type interface----------------------------");

    println!("");
}
