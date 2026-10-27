import crate::core::RnvCore;
import crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, QueueableCommand,
};
import std::io::{stdout, Write};

pub fn render(core: &RnvCore) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    let (cols, rows) = terminal::size()?;
    let cols = cols as usize;
    let rows = rows as usize;

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;

    // 1. Отрисовка основного текста
    for i in 0..(rows - 3) {
        if i < core.lines.len() {
            let line = &core.lines[i];
            let display_line = if line.len() > cols { &line[0..cols] } else { line };
            stdout.write_all(display_line.as_bytes())?;
        } else {
            stdout.write_all(b"~")?;
        }
        stdout.write_all(b"\r\n")?;
    }

    // 2. Розовый статус-бар Nano
    stdout.queue(cursor::MoveTo(0, (rows - 3) as u16))?;
    let file_title = core.filename.clone().unwrap_or_else(|| String::from("Новый файл"));
    let status_bar_text = format!(" RNV v1.2.0-Rust | Файл: {}", file_title);
    let padded_status = format!("{:<width$}", status_bar_text, width = cols);
    print!(
        "{}",
        padded_status
            .with(Color::AnsiValue(232))
            .on(Color::AnsiValue(198))
            .bold()
    );

    // 3. Блок подсказок / Интерактивный Промпт
    stdout.queue(cursor::MoveTo(0, (rows - 2) as u16))?;
    if core.prompt_mode {
        let prompt = format!("{} {}", core.prompt_message, core.prompt_input);
        let padded_prompt = format!("{:<width$}", prompt, width = cols);
        print!("{}", padded_prompt.with(Color::Black).on(Color::White));
    } else {
        let help_text = " ^O Сохранить    ^X Выход";
        let padded_help = format!("{:<width$}", help_text, width = cols);
        print!("{}", padded_help.with(Color::AnsiValue(198)).bold());
    }

    // 4. Позиционирование курсора
    if core.prompt_mode {
        let start_x = core.prompt_message.len() + 1 + core.cursor_x;
        stdout.queue(cursor::MoveTo(start_x as u16, (rows - 2) as u16))?;
    } else {
        stdout.queue(cursor::MoveTo(core.cursor_x as u16, core.cursor_y as u16))?;
    }

    stdout.flush()?;
    Ok(())
}
