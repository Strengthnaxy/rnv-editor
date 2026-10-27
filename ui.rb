module UI
  def self.start(filename)
    core = RnvCore.new(filename)
    $stdout.sync = true

    $stdin.noecho do
      $stdin.raw do
        loop do
          render(core)
          handle_input(core)
        end
      end
    end
  ensure
    # Полный сброс стилей терминала при выходе
    print "\e[?25h\e[0m\e[2J\e[H"
  end

  def self.render(core)
    rows, cols = IO.console.winsize
    buffer = "\e[?25l\e[H" # Прячем курсор, прыгаем в начало

    # Отрисовка текстового буфера
    (0...rows - 2).each do |i|
      if i < core.lines.size
        line = core.lines[i][0...cols] || ""
        buffer << line << "\e[K\n"
      else
        buffer << "~\e[K\n"
      end
    end

    # Плавное вычисление статус-бара в самом низу
    buffer << "\e[#{rows - 1};1H"
    mode_str = core.mode == :insert ? " -- INSERT -- " : " -- COMMAND -- "
    status_text = " RNV #{mode_str}| Файл: #{core.filename || 'Новый'}"
    buffer << "\e[48;5;198m\e[38;5;232m\e[1m" << status_text.ljust(cols) << "\e[0m"

    # Возврат курсора на рабочую позицию
    buffer << "\e[#{core.cursor_y + 1};#{core.cursor_x + 1}H\e[?25h"
    print buffer
  end

  def self.handle_input(core)
    char = $stdin.getc
    case char
    when "\e"
      core.mode = :command
    when "\u0018" # Ctrl+X
      exit
    when "\u000F" # Ctrl+O
      core.save_file
    else
      if core.mode == :command
        case char
        when 'i' then core.mode = :insert
        when 'h' then core.move_cursor(-1, 0)
        when 'j' then core.move_cursor(0, 1)
        when 'k' then core.move_cursor(0, -1)
        when 'l' then core.move_cursor(1, 0)
        end
      else
        case char.ord
        when 127, 8 then core.delete_char
        when 13, 10 then core.insert_newline
        else
          core.insert_char(char) if char.match?(/[[:print:]]/)
        end
      end
    end
  end
end
