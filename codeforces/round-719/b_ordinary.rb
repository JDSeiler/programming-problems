# @type [Integer]
t = gets.chomp.to_i

def test_number
  # @type [Integer]
  n = gets.chomp.to_i

  if n < 10
    n
  else
    num_sets = (Math.log10 n).floor

    (9 * num_sets) + count_remaining_normals(n, num_sets + 1)
  end
end

def count_remaining_normals(number, num_digits)
  # @type [String]
  digits = number.to_s

  close_guess = digits[0].to_i
  nearest_boundary = Array.new(num_digits, digits[0]).join.to_i

  if digits.chars.uniq.length == 1 || number > nearest_boundary
    # Number is ON the boundary, 222 for example
    close_guess
  else
    # Number is under the boundary, 200 for example
    close_guess - 1
  end
end

(1..t).each do
  puts test_number
end
