use std::mem;

fn main() {

//TUPLES

let mut tpl = (500, "hi", true);

tpl.0 = 200;                    //auf tuple field zugreifen


// println!("{tpl}");            // std::fmt::Display

println!("{tpl:?}");            // std::fmt::Debug

let (x, y, z) = tpl;        //tuple unpacking, macht nur eine kopie

println!("{x}");
println!("{y}");
println!("{z}");


//ARRAYS

let array = [1, 2, 3];

println!("{:?}", array);
println!("{}", array[0]);
println!("{}", array[1]);
println!("{}", array[2]);

let array2:[i32; 100] = [1337; 100];


//ARRAY SLICES


let xs: [i32; 6] = [0, 1, 2, 3, 4, 5];      //6 x 32bit int (32bit = 4 byte), 6 x 4 Byte = 24 byte

println!("length: {}", xs.len());
println!("data size: {}", mem::size_of_val(&xs));

println!("{:?}", &xs[1..4]);        //EXKLUSIVE index 4     = [1, 2, 3]
println!("{:?}", &xs[1..=4]);        //INKLUSIVE index 4    = [1, 2, 3, 4]




//STRINGS

let hello = String::from("Hello World!");

for b in hello.bytes(){         //printed die byte werte der einzelnen buchstaben
    println!("{}", b);
}


for c in hello.chars(){         //printed die einzelnen chars
    println!("{}", c);
}


//IF ELSE
let number = 10;
if number > 5 {
    println!("Nummer größer 5");
} else {
    println!("Nummer kleiner 5");
}

//TERNARY
let condition = true;
let number = if condition {5} else {8};
println!("{number}");


//LOOPS

println!("----LOOOPS-----");

//FOR LOOP
//let array = [0, 1, 2, 3];        //geht auch mit array
for num in 0..=11{           //0 bis exklusive 11
    print!(" {num}");
}

//WHILE LOOOOP
let mut counter = 0;
'while_name: while counter < 10{             //loop kann auch gelabelt werden
    counter += 1;
    println!("{}", counter);
    if counter == 7{
        break 'while_name;
    }
}


//"normaler LOOP"
loop {      //quasi endlosschleife, braucht break!
    counter -= 1;
    println!("loop {counter}");
    if counter == -5{
        break;
    }
}

//kann auch einen wert returnen:
let result = loop{
    counter += 1;
    println!("loop2 {counter}");
    if counter == 3{
        break 15;               //<- 15 wird returnt
    }
};
println!("{}", result);





//FUNKTIONEN    

// println!("Meine Funktion: {}",meine_funktion(5));
meine_funktion(5);

      //           |übergabeparam. mit typ!    |return type
fn meine_funktion(mut input: i32)               -> i32 {       
    input += 100;
    input                   // ausdruck ohne semikolon = indirekter return
    //return input;         //geht auch direkt mit return keyword
}


//MATCH (wie switch)
let num = 5;

match num {
    1 => println!("Die Zahl ist eine 1"),
    2 => {                                          //mehrzeilige cases mit {}
        println!("Die Zahl ist eine 2");
        println!("2 ist eine tolle Zahl");
    },
    3 | 4 | 5 | 6 => println!("Die Zahl ist 3-6"),
    _ => println!("DEFAULT"),                       //default wird mit _ gekennzeichnet
}

//return wert von match kann auch in einer variable gespeichert werden:

let boolean = true;

let binary = match boolean{
    false => 0,
    true => 1,
};

println!("{boolean} ist in binär: {binary}");




//ENUMS

#[derive(Debug)]        //benötigt, damit instanzen des Enums mit dem debug formatter geprinted werden können
enum PermissionLevel {
    User,
    Instructor,
    Admin
}

//enum funktion 
impl PermissionLevel{
    fn description(&self) -> String {

        match self {                    //match == switch (hier wird self geprüft)      wird verwendet, wenn alle cases(user, admin, instr) abgedeckt werden sollen
            PermissionLevel::User => String::from("I AM AN USER"),                  //arm User (case)
            PermissionLevel::Instructor => String::from("I AM AN Instructor"),      //arm Instructor
            PermissionLevel::Admin => String::from("I AM AN ADMIN"),                //arm Admin
        }
    }

    //IF LET
    fn is_admin(&self) -> bool{
        let returnvalue = if let PermissionLevel::Admin = self {        //kann gelesen werden wie (PermissionLevel::Admin == self) wird aber nur mit einem = geschrieben (wtf)
            true
        } else {
            false
        };

        returnvalue
    }



}


let user1 = PermissionLevel::User;
let user2 = PermissionLevel::Instructor;
let user3 = PermissionLevel::Admin;

println!("{:?}", user1);
println!("{}", user1.description());


println!("who is the admin?");
println!("{}", user1.is_admin());
println!("{}", user2.is_admin());
println!("{}", user3.is_admin());


//es können auch Datentypen wie int oder String in emun gespeichert werden:
#[derive(Debug)]
enum LoginData{
    None,
    Username(String)
}

let data1 = LoginData::None;
let data2 = LoginData::Username(String::from("name_of_user"));


println!("{:?}", data2);



//OPTION ENUM

// enum Option<T>{
//     Some(T),
//     None,
// }

let x: u32 = 5;
let y: Option<u32> = Some(5);


fn add(x: u32, y: Option<u32>) -> u32{
    match y {
        Some(val) => x + val,
        None => x + 0,
    }   
}

println!("x + y = {}", add(x, y));



//WHILE LET
let mut nums = 0..=10;

//wie   (Some(num) == nums.next())
while let Some(num) = nums.next(){
    println!("{num}");
} 


//geht auch mir Array
let mut num_array = [1, 2, 3].into_iter();

while let Some(num) = num_array.next(){
    println!("{num}");
}



//STRING SLICES


fn first_word(input: &str) -> &str{
    let bytes = input.as_bytes();

    for(i, &character) in bytes.iter().enumerate(){
        if character == b' '{
            return &input[0..i];
        }
    }

    input       //wird returnt, wenn input kein leerzeichen hat
}

println!("{}", first_word("hallo_das_ist ein input!"));


//STRUCTS   (wie klassen)

#[derive(Debug)]
struct User{
    is_admin: bool,
    username: String,
    password: String,
}

fn build_admin(username: String, password: String) -> User{
    return User {
        is_admin: true,
      //username: username     kann abgekürzt werden
        username,           //wenn übergabeparam == fieldname ist, muss hier nicht username: username stehen
        password,
    };
}

let user1 = User{
    is_admin: true,
    username: String::from("fritz"),
    password: String::from("123qwe"),
};

println!("{:?}", user1);
println!("{}", user1.is_admin);
println!("{}", user1.username);
println!("{}", user1.password);

let user2 = build_admin(String::from("Hans"), String::from("12345"));
println!("{:?}", user2);



}


