def read_test_case
  # @type [String]
  line = gets.chomp
  # &: notation is sometimes called 'pretzel colon'
  # Kind of like a method references? It's syntax sugar for some Proc magic:
  # &:to_i == { |foo| foo.to_i }
  # :to_i here is a symbol, and:
  line.split(' ').map(&:to_i)
end

# @param nums [Array]
def get_diff_array(nums)
  nums.map.each_with_index do |num, idx|
    # Problem uses 1 indexing, so I'll be consistent
    num - (idx + 1)
  end
end

# @param diffs [Array]
def count_diff_array(diffs)
  counts = {}
  diffs.each do |diff|
    if !counts.include? diff
      counts[diff] = 1
    else
      counts[diff] += 1
    end
  end
  counts
end

# @param counts [Hash]
def count_total_pairs(counts)
  pairs = 0
  counts.each_value do |count|
    pairs += compute_triangular_number(count - 1)
  end
  pairs
end

def compute_triangular_number(n)
  (n * (n + 1)) / 2
end

# @type[Integer]
t = gets.chomp.to_i

(1..t).each do
  _test_size = gets

  test = read_test_case
  diff_array = get_diff_array test
  counts = count_diff_array diff_array
  puts count_total_pairs counts
end
