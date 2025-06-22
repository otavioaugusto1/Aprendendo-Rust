fn main() {
    println!("Hello, world!");
    println!("Teste 2!");
    println!("teste 3!");
    print!("Não quebra!");
    print!("linha!");

    // Isso é um comentário

    let nome = "Otávio";
    println!("O meu nome é: {}", nome);

    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string

    // Posso declarar dessa forma
    println!("{}",my_num);
    println!("{}",my_text);
    println!("{}",my_letter);
    println!("{}",my_bool);
    println!("{}",my_double);

    // Ou dessa forma:
    /* 
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string 
    // */


    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);
    println!("5 * 10: {}", a * b);
    println!("5 + 10: {}", a + b);

    let eh_usuario = true;
    let eh_admin = false;
    println!("Possui acesso privilegiado? {}", eh_usuario && eh_admin);
    println!("Possui acesso privilegiado? {}", eh_usuario && !eh_admin);

    // if/else if/else
    if 7 > 5 {
        println!("7 é maior que 5.");
    }

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
    
    let tempo = 20;
    let saudacoes = if tempo < 18{
        "bom dia"
    } else {
        "boa tarde"
    };
    println!("{}", saudacoes);

    //match
      let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }

  // multiplos matchs

  let dia = 3;

  match dia {
    1 | 2 | 3 | 4 | 5 => println!("Dia da semana"),
    6 | 7 => println!("Final de semana"),
    _ => println!("número inválido"),
  }
    
    // match com retorno 
    /*   
    let day = 4;

    let resultado: $str = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    }
*/


let mut contador = 1;

loop {
  println!("Hello World!");

  if contador == 3 {
    break;
  }

  contador += 1;
}

// loop com retorno 
/*

let mut contador = 1;

let result = loop {
  println!("Hello World!");

  if contador == 3 {
    break count;
  }

  contador += 1;
}
*/

// while
/*

let mut cont1 = 1;

while cont1 <= 5{
  println!("Hello, Otávio. Contador no valor {}!", cont1);
  //posso utilizar break para sair de um loop while, por mais que nesse exemplo ele não entre com o valor 6
  if cont1 == 6 {
    break;
  }
  cont1 += 1;
}
*/
  println!("While loop ");
  let mut cont1 = 1;

  while cont1 <= 2{
    println!("Hello, Otávio. Contador no valor {}!", cont1);
    //posso utilizar break para sair de um loop while, por mais que nesse exemplo ele não entre com o valor 6
    if cont1 == 6 {
      break;
    }
    cont1 += 1;
  }

  println!("For loop ");
  for i in 1..6 {
    println!("i is: {}", i);
  }

  println!("For loop com final incluso ");
  for i in 1..=6 {
    println!("i is: {}", i);
  }

  // Funções
  fn soma(v1: i32, v2: i32) -> i32{
    return v1 + v2;
  }
  let sum: i32 = soma(1,2);
  println!("Soma é: {}", sum);

  let str1: &str = "oi, &str";
  let mut str2 = "oi, to_string".to_string();
  let mut str3 = String::from("oi,String::From");

  println!("{}",str1);
  println!("{}",str2);
  println!("{}",str3);

  //Concatenando
  println!("Concatenando");
  let mut str4 = String::from("Hello");
  println!("{}", str4);
  str4.push_str(" world");
  println!("{}", str4);

  let s1 = String::from("Hello");
  let s2 = String::from("World!");
  let s3 = String::from("What a beautiful day!");
  let result = format!("{} {} {}", s1, s2, s3);
  println!("{}", result);

  // Tamanho do nome
  println!("Tamanho do nome com .len()");
  let tamanho_do_nome = String::from("John");
  println!("Tamanho: {}", tamanho_do_nome.len()); // 4

  
}