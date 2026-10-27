require 'io/console'

class RnvCore
  attr_accessor :lines, :cursor_x, :cursor_y, :mode, :filename

  def initialize(filename = nil)
    @filename = filename
    @lines = (filename && File.exist?(filename)) ? File.readlines(filename).map(&:chomp) : [""]
    @lines = [""] if @lines.empty?
    @cursor_x = 0
    @cursor_y = 0
    @mode = :command
  end

  def save_file
    return unless @filename
    File.open(@filename, 'w') { |f| f.puts @lines.join("\n") }
  end

  def move_cursor(dx, dy)
    @cursor_y = [[0, @cursor_y + dy].max, @lines.size - 1].min
    @cursor_x = [[0, @cursor_x + dx].max, @lines[@cursor_y].size].min
  end

  def insert_char(char)
    @lines[@cursor_y].insert(@cursor_x, char)
    @cursor_x += 1
  end

  def delete_char
    if @cursor_x > 0
      @lines[@cursor_y].slice!(@cursor_x - 1)
      @cursor_x -= 1
    elsif @cursor_y > 0
      @cursor_x = @lines[@cursor_y - 1].size
      @lines[@cursor_y - 1] += @lines.delete_at(@cursor_y)
      @cursor_y -= 1
    end
  end

  def insert_newline
    remainder = @lines[@cursor_y].slice!(@cursor_x..-1) || ""
    @lines.insert(@cursor_y + 1, remainder)
    @cursor_y += 1
    @cursor_x = 0
  end
end
