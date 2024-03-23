use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    let num_secreto = rand::thread_rng()
        .gen_range(1..=100);

    // println!("O número secreto é: {num_secreto}");

    loop{
        println!("Por favor, insira seu palpite.");
        
        let mut palpite = String::new();
    
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a linha");
    
        let palpite: u32 = match palpite
            .trim()
            .parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERRO: insira um número válido.");
            continue
        },
        };
    
        println!("Seu palpite: {palpite}");
    
        match palpite.cmp(&num_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }

}
