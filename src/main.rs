// best practice di rust name function harus huruf kecil semua
// fn main() {
//     println!("Hello World");
//     println!("Hello Eko");
//     println!("Hello Budi");
// }
use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: isize  = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index < 0 {
        println!("The value less more than 0");
        return;
    } else if (index as usize) >= a.len() {
        println!("The value more than array length");
        return;
    } else {
        let element = a[index as usize];
        println!("The value of the element at index {index} is: {element}");
        return;
    }
}

// Unit Test
#[test] // decorator(di typescript)/annotation(di java), kalo di rust namanya attribute
fn hello_test(){ 
    println!("Hello Test");
}  // --> cara jalankan unit test


#[test]
fn testing(){
    println!("Hello tester");
}

#[test]
fn test_variable() {
    let name = "Eko Kurniawan Khannedy";
    let mut name2 = "Sheila Iga Sirait";
    println!("{name2}");
    name2 = "Farah Amirah Abidin";
    println!("Hello {}, {}", name, name2); // {} --> string interpolation di java / string substitute / string template
}

#[test]
fn static_typing() {
    let name = "Sheila Iga Sirait";

    // name = 10;   ---> error
    println!("Hello {}", name); // {} --> string interpolation di java / string substitute / string template
}

#[test]
fn shadowing() {
    let name = "Sheila Iga Sirait";
    println!("Hello {}", name); // variable ini akan tertutupi

    let name = 10; // --> shadowing variable
    println!("Angka {}", name); 

    println!("{}", name);
}


/*
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
*/
#[test]
fn comment(){
    // ini komentar
    println!("Hello"); // ini komentar
}

#[test]
fn explicit(){
     let age: i32 = 20;
     println!("{}", age);
}

#[test]
fn data_type(){
     let a: i8 = 10;
     println!("{}", a);

     let b: f32 = 10.5;
     print!("{}", b);
}

#[test]
fn number_conversion(){
     let a: i8 = 10;
     println!("{}", a);

     let b: i16 = a as i16;
     println!("{}", b);

     let c: i32 = b as i32;
     println!("{}", c);

     let d: i64 = 1000000000;
     let e: i8 = d as i8;
     println!("{}", e);

}

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a /  c;
    let e = a + b;

    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison(){
    let a = 20;
    let b = 20;

    let result = a >= b; // lebih dari atau sama dengan
    println!("{}", result);
}


#[test]
fn boolean_operator(){
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);

}

#[test] 
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple(){
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a, b, c);
}  


#[test]
fn tuple_destructuring(){
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, _c) = data;

    println!("{} {} ", a, b);
} 

#[test]
fn tuple_mutable() {
    let mut data: (i32, f64, bool) = (20, 12.6, false);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("destructuring : {} {} {}", a, b, c);

    data.0 = 11;
    data.1 = 11.11;
    data.2 = true;
    println!("data updated:, {:?}", data);
}

#[test]
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);

}


// from doc
#[test]
fn doc_array () {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", 
        "August", "September", "October", "November", "December" ];
}
