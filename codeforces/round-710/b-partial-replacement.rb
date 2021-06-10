def solve_case
  len, k = gets.chomp.split(' ').map(&:to_i)
  str = gets.chomp.split('')
  star_locations = []
  str.each_with_index do |c, idx|
    if c == '.'
      # do nothing
    else
      star_locations.push(idx)
    end
  end
  # I WANT the edges... Want to find the furthest star
  # away such that the star is still within k.
end

t = gets.chomp.to_i
t.times do
  solve_case
end
