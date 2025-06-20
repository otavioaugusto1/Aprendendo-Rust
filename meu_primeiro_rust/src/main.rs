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

}
