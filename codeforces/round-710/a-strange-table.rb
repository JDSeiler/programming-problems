def solve_case
  n, m, x = gets.chomp.split(' ').map(&:to_i)

  c = (x.to_f / n.to_f).ceil
  r = x - (n * (c - 1))

  row_x = c + (m * (r - 1))
  puts row_x
end

t = gets.chomp.to_i
t.times do
  solve_case
end
