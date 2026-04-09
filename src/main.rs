extern crate rand;
use rand::distr::{Alphanumeric, SampleString};
use random_string::generate;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Escolha:\n 1. Senha apenas com caracters\n 2. Senha com caracteres e simbolos");
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um numero valido");
            return;
        }
    };
    match num {
        1 => senha_normal(),
        2 => senha_simbolo(),
        _ => println!("Invalido"),
    }
}

fn senha_normal() {
    println!("Qual tamanho da senha que voce vai querer?");
    let mut tamanho = String::new();
    io::stdin().read_line(&mut tamanho).unwrap();
    let tamanho: usize = match tamanho.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um numero valido");
            return;
        }
    };
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), tamanho);
    println!("{string}");
}

fn senha_simbolo() {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+";

    println!("Qual tamanho da senha que voce vai querer?");
    let mut tamanho = String::new();
    io::stdin().read_line(&mut tamanho).unwrap();
    let tamanho: usize = match tamanho.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um numero valido");
            return;
        }
    };

    println!("{}", generate(tamanho, charset));
}
