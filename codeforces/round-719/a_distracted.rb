require 'set'

t = gets.chomp.to_i

def cannot_be_sus
  _ = gets
  # @type [String]
  input = gets.chomp

  last_seen_char = nil
  seen = Set.new
  input.each_char do |c|
    if c == last_seen_char
      # do nothing
    elsif seen.include? c
      return false
    else
      seen.add c
    end
    last_seen_char = c
  end
  true
end

(1..t).each do
  cannot_be_sus ? (puts 'YES') : (puts 'NO')
end
