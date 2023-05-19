Ejercicio en Rust

//Escribe un programa en Rust que le pida al usuario que ingrese dos números enteros, 
//calcule su suma e imprima el resultado en la consola en Rust



use std::io;

fn main() 
{
    println!("Ingrese el primer número:");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Error al leer el número.");

    // Convertir el primer número a un entero
    let num1: i32 = match input_1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("¡Debe ingresar un número válido!");
            return;
        }
    };

    println!("Ingrese el segundo número:");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Error al leer el número.");

    let num2: i32 = match input_2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("¡Debe ingresar un número válido!");
            return;
        }
    };

    let suma = num1 + num2;

    // Mostrar el resultado en la consola
    println!("La suma de {} y {} es: {}", num1, num2, suma);
}