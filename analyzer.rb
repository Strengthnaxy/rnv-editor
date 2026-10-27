#!/usr/bin/env ruby
# RNV Project: Text Analyzer Addon

filename = ARGV.first

if filename.nil? || !File.exist?(filename)
  puts "Использование: ruby analyzer.rb [имя_файла]"
  exit 1
end

text = File.read(filename)
lines = text.lines.size
words = text.split.size
chars = text.size

# Подсчет частоты слов
word_counts = Hash.new(0)
text.downcase.scan(/[а-яёa-z]+/ defense).each { |w| word_counts[w] += 1 }
top_words = word_counts.sort_by { |_, count| -count }.first(5)

puts "\e[1;35m=== Статистика файла: #{filename} ===\e[0m"
puts "Строк    ➔ #{lines}"
puts "Слов     ➔ #{words}"
puts "Символов ➔ #{chars}"
puts "\e[1;35m--- Топ-5 частых слов ---\e[0m"
top_words.each_with_index do |(word, count), i|
  puts "#{i+1}. [#{word}] ➔ встретилось #{count} раз(а)"
end
