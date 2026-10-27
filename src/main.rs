mod core;
mod ui;

import core::RnvCore;
import crossterm::event::{self, Event, KeyCode, KeyModifiers};
import crossterm::terminal;
import std::env;

fn print_help() {
    println!("RNV (Ruby Nano Vim) -> Теперь на чистом Rust!");
    println!("\nИспользование:\n  rnv [флаги] [имя_файла]");
    println!("\nФлаги:\n  -h, --help     Показать справку\n  -v, --version  Показать версию");
    println!("\nУправление (Полный стиль Nano):\n  Пишите текст сразу без режимов.");
    println!("  Ctrl + O       Сохранить файл (с интерактивным запросом)");
    println!("  Ctrl + X       Выйти из редактора");
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
        return Ok(());
    }
    if args.contains(&String::from("-v")) || args.contains(&String::from("--version")) {
        println!("RNV Text Editor v1.2.0-rust-stable");
        return Ok(());
    }

    let filename = args.iter().skip(1).find(|arg| !arg.starts_with('-')).cloned();
    let mut core = RnvCore::new(filename);

    terminal::enable_raw_mode()?;

    loop {
        ui::render(&core)?;

        if let Event::Key(key_event) = event::read()? {
            if core.prompt_mode {
                // Обработка ввода внутри диалога сохранения (Prompt)
                match key_event.code {
                    KeyCode::Enter => {
                        if !core.prompt_input.trim().is_empty() {
                            core.filename = Some(core.prompt_input.clone());
                            let _ = core.save_file();
                        }
                        core.prompt_mode = false;
                        core.prompt_input.clear();
                        // Возвращаем курсор обратно на текст файла
                        core.cursor_y = 0;
                        core.cursor_x = core.lines[0].len();
                    }
                    KeyCode::Esc => {
                        core.prompt_mode = false;
                        core.prompt_input.clear();
                    }
                    KeyCode::Char(c) => core.insert_char(c),
                    KeyCode::Backspace => core.delete_char,
                    _ => {}
                }
            } else {
                // Основной режим редактирования (Прямой ввод текста как в Nano)
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    match key_event.code {
                        KeyCode::Char('x') => break, // Ctrl+X -> Выход
                        KeyCode::Char('o') => {     // Ctrl+O -> Сохранить с вопросом
                            core.prompt_mode = true;
                            core.prompt_message = String::from("Имя файла для записи?");
                            core.prompt_input = core.filename.clone().unwrap_or_default();
                            core.cursor_x = core.prompt_input.len();
                        }
                        _ => {}
                    }
                } else {
                    match key_event.code {
                        KeyCode::Char(c) => core.insert_char(c),
                        KeyCode::Backspace => core.delete_char,
                        KeyCode::Enter => core.insert_newline(),
                        KeyCode::Left => core.move_cursor(-1, 0),
                        KeyCode::Right => core.move_cursor(1, 0),
                        KeyCode::Up => core.move_cursor(0, -1),
                        KeyCode::Down => core.move_cursor(0, 1),
                        _ => {}
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!("\x1B[2J\x1B[H"); // Очистка экрана при выходе
    Ok(())
}
