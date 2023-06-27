//Для сжатия и распоковки потоков используем библиотеку flate2

use flate2::write::GzEncoder;
use flate2::Compression;
//Принимаем имя файла от пользователя, который должен быть сжат
use std::env::args;
//Так так работаем с файлами, то используем fs
use std::fs::File;
//Читаем содежимое файла
use std::io::BufReader;
//Копируем содержимое изначального файла в сжатый
use std::io::copy;
//Время затраченное на сжатие файла
use std::time::Instant;

fn main() {
    //Если длина аргумента не равна 3, то работа программы заканчивается из-за возращение return. Где source - изначальный файл, а target - сжатый
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    //Просматриваем аргументы, и выбираем первый аргумент, который находится в nth(1)
    //File::open(args().nth(1).unwrap()).unwrap() - является аргументом для исходного файла source
    //Файл будет прочтен с помощью BufReader
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //Вывод сжатого файла
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //Здесь начинается кодирование для сжатия исходного файла, чтобы получить target
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    //Показывает сколько времени прошло для сжатия файла
    println!("Elapsed: {:?}", start.elapsed());
}