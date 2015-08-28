// по мотивам поста http://habrahabr.ru/post/250817/
extern crate rand;
use std::io;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use std::str::FromStr;

fn read_line<T: FromStr>( text: &str ) -> Result<T, T::Err> {
    let mut buffer = String::new();
    print!( "{}", text );
    io::stdout().flush()
        .ok()
        .expect( "Не удалось очистить stdout!" );
    io::stdin().read_line( &mut buffer )
        .ok()
        .expect( "Невозможно прочитать строку!" );
    buffer.trim().parse::<T>()
}

fn main() {
    let max_prisoners: usize = read_line( "Введите количество заключенных: " ).unwrap_or( 0 );
    let max_attempt: usize = read_line( "Введите количество попыток: " ).unwrap_or( 0 );
    let max_experiment = read_line( "Введите количество экспериментов: " ).unwrap_or( 0 );
    let mut life_counter: i16 = 0;
    let mut death_counter: i16 = 0;
    for _ in ( 0 .. max_experiment ) {
        let mut number: Vec< usize > = ( 0 .. max_prisoners ).collect();
        let mut pbox: Vec< usize > = Vec::new();
        let mut life_status: bool = false;
        // перемешиваем номера
        thread_rng().shuffle( &mut *number );
        for i in ( 0 .. max_prisoners ) {
            // поместим номер из массива в коробку
            pbox.push( number[i] );
        }
        // цикл по всем заключенным
        for prisoner in ( 0 .. max_prisoners ) {
            let mut next = prisoner;
            // цикл по количеству попыток
            for _ in ( 0 .. max_attempt ) {
                // если в коробке номер заключенного,
                // то переходим к следующему
                if pbox[next] == prisoner {
                    life_status = true;
                    break;
                } else {
                    // иначе продолжаем открывать коробки
                    next = pbox[next];
                    life_status = false;
                }
            }
        }
        match life_status {
            true  => life_counter  += 1,
            false => death_counter += 1
        }
    }
    println!( "Количество экспериментов {}", max_experiment );
    println!( " > Количество выживаний - {} [ {} % ]", life_counter,
              ( life_counter * 100 ) as f32 / max_experiment as f32 );
    println!( " > Количество смертей   - {} [ {} % ]", death_counter,
              ( death_counter * 100 ) as f32 / max_experiment as f32 );
}
