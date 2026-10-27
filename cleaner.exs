# RNV Project: Automated System Cleaner Addon (Elixir Version)

defmodule RnvCleaner do
  def clean_backups do
    IO.puts("\e[1;35m[RNV-Cleaner] Запуск сканирования системы...\e[0m")
    
    # Ищем скрытые бэкапы нашего редактора RNV (.вместо_имени.txt.bak)
    Path.wildcard(".*.bak")
    |> Enum.each(fn file ->
      File.rm(file)
      IO.puts("\e[32m➔ Удален старый бэкап редактора: #{file}\e[0m")
    end)

    # Ищем мусорные файлы конфигурации или краш-логи от беты AIMP
    Path.wildcard("~/.config/aimp/*.log")
    |> Enum.each(fn log_file ->
      File.rm(log_file)
      IO.puts("\e[32m➔ Очищен лог плеера AIMP: #{log_file}\e[0m")
    end)

    IO.puts("\e[1;32m[RNV-Cleaner] Процесс очистки успешно завершен!\e[0m")
  end
end

RnvCleaner.clean_backups()
