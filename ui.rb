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
    print "\e[?25h\e[0m\e[2J\e[H"
  end

  def self.render(core)
    print "\e[2J\e[H\e[?25l"
    
    core.lines.each do |line|
      puts line
    end

    # Статус-бар в розовом цвете Gentoo/Magenta
    print "\e[H\e[#{IO.console.winsize - 1}H"
    mode_str = core.mode == :insert ? " -- INSERT -- " : " -- COMMAND -- "
    print "\e[48;5;198m\e[38;5;232m\e[1m RNV \e[0m\e[7m#{mode_str}| Файл: #{core.filename || 'Новый'} \e[0m"

    print "\e[#{core.cursor_y + 1};#{core.cursor_x + 1}H\e[?25h"
  end

  def self.handle_input(core)
    char = $stdin.getc
    case char
    when "\e"
      core.mode = :command
    when "\u0018" # Ctrl+X
      print "\e[2J\e[H"
      exit
    when "\u000F" # Ctrl+O
      core.save_file if core.mode == :command
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
