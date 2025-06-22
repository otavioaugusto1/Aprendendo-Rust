# Aprendendo-Rust
Usarei esse repositório para documentar meu aprendizado em Rust

No Rust, **Cargo** é a ferramenta de gerenciamento de pacotes e build system (sistema de compilação). Ele é responsável por facilitar o processo de criação, construção, execução, teste e gerenciamento de dependências de projetos Rust. Cargo é uma parte fundamental do ecossistema Rust, e basicamente, automatiza muitas das tarefas envolvidas no ciclo de vida de desenvolvimento de um projeto.

### Principais Funções do Cargo:

1. **Gerenciamento de dependências**: Cargo pode baixar e gerenciar bibliotecas de terceiros (chamadas de crates) automaticamente para o seu projeto.
2. **Compilação**: Cargo compila o código-fonte do seu projeto de maneira otimizada.
3. **Testes**: Cargo pode rodar testes automatizados e garantir que o código está funcionando como esperado.
4. **Criação de pacotes (crates)**: Cargo facilita a criação e publicação de bibliotecas Rust para o repositório oficial (crates.io).
5. **Facilidade de execução**: Com Cargo, você pode rodar seu código de forma simples.

### Comando `cargo init`

O comando `cargo init` é usado para inicializar um novo projeto Rust. Ele cria a estrutura básica de um projeto e configura tudo para você, incluindo o arquivo `Cargo.toml`, que é onde as dependências e configurações do projeto são armazenadas.

Ao rodar o comando `cargo init` em um diretório, o Cargo vai:

* Criar um diretório com o nome do projeto (se não existir).
* Adicionar um arquivo `Cargo.toml` para configurações do projeto, incluindo dependências.
* Criar uma pasta `src/` com um arquivo `main.rs` contendo o código inicial.

### Exemplos:

* **Inicializar um novo projeto**:

  ```bash
  cargo init meu_projeto
  ```

* **Inicializar um novo projeto sem o diretório (para projetos de bibliotecas)**:

  ```bash
  cargo init --lib
  ```

Se você não especificar o nome do projeto, o `cargo init` usará o nome do diretório atual como nome do projeto.

### Estrutura do Projeto Criado

Após rodar o comando, a estrutura básica do seu projeto será algo assim:

```
meu_projeto/
│
├── Cargo.toml       # Arquivo de configuração do projeto
└── src/
    └── main.rs      # Código-fonte inicial
```

O arquivo `Cargo.toml` contém informações como o nome do pacote, versão e dependências.

Em resumo, `cargo init` é uma maneira rápida e fácil de configurar a base do seu projeto em Rust, já configurando o ambiente básico para você começar a programar.

### Execução do código em Rust

Para executar, basta ir na raiz do projeto (onde tem o Cargo.toml e o src/) e executar:

```
cargo run
```

### Macros

Uma macro é como uma função, mas com um ponto de exclamação (!) depois. Por enquanto, saiba apenas que macros são semelhantes a funções (elas executam coisas), mas nem sempre seguem as mesmas regras que as funções.

### Comentários

Um comentário de uma linha é assim:

```
// Comentário de uma linha
```

Um comentário de múltiplas linhas:

```
/* The code below will print the words Hello World!
to the screen, and it is amazing */
```

### Variáveis

Variáveis são definidas com o prefixo "let", exemplo:

```
let name = "Otávio";
println!("Meu nome é {}", name);
```

Por padrão, variáveis em Rust não podem ser alteradas após a definição.

```
let x = 5;
x = 6; // erro
```

Se você quiser alterar o valor dessa variável posteriormente, precisa adicionar a keyword "mut" entre o let e o nome da variável:

```
let mut x = 5;
x = 6;
println!("O valor de x é: {}", x);
```

Ao contrário de muitas outras linguagens de programação, as variáveis ​​em Rust não precisam ser declaradas com um tipo específico (como "String" para texto ou "Int" para números, se você estiver familiarizado com as linguagens de C ou Java).

Em Rust, o tipo de uma variável é determinado pelo valor que você atribui a ela. Rust analisa o valor e escolhe automaticamente o tipo correto:

```
let my_num = 5;         // integer
let my_double = 5.99;   // float
let my_letter = 'D';    // character
let my_bool = true;     // boolean
let my_text = "Hello";  // string
```

No entanto, é possível informar explicitamente ao Rust qual tipo um valor deve ser:

```
let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // float
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello";  // string
```

Para criar uma constante, use a palavra-chave const, seguida do nome, tipo e valor (não podemos deixar de especificar o tipo da variável const)

```
const DATA_NASCIMENTO: int32 = 2000;
```

### Operadores

Eles são: 

* Aritméticos: +, -, /, *
* Comparação: = =, >=, <=, <, >, !=
* Atribuição: =, +=, -=, *=, /=, %=
* Lógicos: && (AND), || (OR), ! (NOT)

Consigo definir um valor booleano a uma variável sem especificar TRUE ou FALSE

```
let age = 20;
let can_vote = age >= 18;

println!("Consigo votar? {}", can_vote);
```

### If conditions

```
if 7 > 5 {
  println!("7 é maior que 5.");
}
```

Usando if...else como uma expressão

```
  let tempo = 20;
  let saudacoes = if tempo < 18{
    "bom dia"
  } else {
    "boa tarde"
  };
  println!("{}", saudacoes)
```

Fazendo uso do Match: ele é uma boa opção quando pode-se ter vários if/else if alinhados:

```
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
```

Múltiplos matchs
```
  let dia = 3;

  match dia {
    1 | 2 | 3 | 4 | 5 => println!("Dia da semana"),
    6 | 7 => println!("Final de semana"),
    _ => println!("número inválido"),
  }
```

Match com retorno

```
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
```

### Loops

loop: roda infinitamente até definirmos um limite ou break :

```
let mut count = 1;

loop {
  println!("Hello World!");

  if count == 3 {
    break;
  }

  count += 1;
}
```

While loop
```
let mut cont1 = 1;

while cont1 <= 5{
  println!("Hello, Otávio. Contador no valor {}!", cont1);

  cont1 += 1;
}
```

For loops:

```
println!("For loop ");
for i in 1..6 {
  println!("i is: {}", i);
}
```

No for loop, a variável "i" não precisa ser declarada anteriormente, Rust toma conta disso. O range é de 1 a 6, porém o 6 não está incluso. Para incluí-lo, basta fazer:

```
println!("For loop com final incluso ");
for i in 1..=6 {
  println!("i is: {}", i);
}
```

### Funções em Rust

Funções são definidas da seguinte maneira:

```
  // Funções
  fn soma(v1: i32, v2: i32) -> i32{
    return v1 + v2;
  }
  let sum: i32 = soma(1,2);
  println!("Soma é: {}", sum);
```

Nota-se que para declarar que a função possui retorno, adicionamos o "->" e falamos o tipo de retorn. 

### Escopo de variáveis

Variáveis ficam disponíveis dentro do seu escopo declarativo. Abaixo mostrarei um exemplo:

```
fn myFunction() {
  let message = "Hello!";
  println!("{}", message);  // Aqui essa variável message está disponível
}

myFunction();
println!("{}", message); // Error - aqui ela não está disponível, pois está fora do escopo.
```

### Diferenças entre Strings

``` 
let saudacoes: &str = "Hello";
``` 

*Tipo: &str (string slice)
*Descrição: Isso é uma referência imutável a uma sequência de caracteres embutida diretamente no binário (literal de string).
*Vantagem: Muito leve e rápida. Ideal quando não precisa modificar a string.
*Limitada: Você não pode modificar ou fazer operações que alterem o conteúdo.

``` 
let text1 = "Hello World".to_string();
```

*Tipo: String
*Descrição: Aqui você está pegando um &str (o literal "Hello World") e convertendo para uma String.
*Vantagem: Agora você tem propriedade sobre a string e pode modificá-la (adicionar, remover, etc.).

```
let text2 = String::from("Hello World");
```

*Tipo: String
*Descrição: Funcionalmente igual ao .to_string(), também converte um &str em uma String.

Concatenando strings:

```
let mut text2 = String::from("Hello");
text2.push_str(" world);
println!("{}, text2");
```

 Outra forma de concatenação:

```
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);
```

Tamanho da String com .len()
```
println!("Tamanho do nome com .len()");
let tamanho_do_nome = String::from("John");
println!("Tamanho: {}", tamanho_do_nome.len()); // 4
```

