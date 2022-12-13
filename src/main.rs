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

    let first_lettther = 'A';
    println!("first_lettther is {}", first_lettther);
    let space = ' ';
    println!("space is ||{}||", space);
    let other_languege_char = 'Ꮔ';
    println!("space is {}", other_languege_char);
    let cat_face = '😻';
    println!("space is {}", cat_face);
}
