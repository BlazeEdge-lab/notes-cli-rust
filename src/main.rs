use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut notes: HashMap<String, String> = HashMap::new();

    loop {
        print!("Введите команду (add, show, exit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "add" => {
                print!("Введите название заметки: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                print!("Введите текст заметки: ");
                io::stdout().flush().unwrap();
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                let content = content.trim().to_string();

                notes.insert(title.clone(), content);
                println!("Заметка \"{}\" добавлена", title);
            }
            "show" => {
                println!("Список заметок:");
                for (title, content) in &notes {
                    println!("---Заголовок: {}---\nТекст: {}\n", title, content)
                }
            },
            "exit" => {
                println!("Программа завершена.");
                break;
            }
            _ => {
                println!("Неизвестная команда. Попробуйте снова.");
            }
        }
    }
}
