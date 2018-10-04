class Hamming
    def self.compute(first, second : String) Int
        raise ArgumentError.new unless first.size == second.size
        first.chars.zip(second.chars).count {|a, b| a != b }
    end
end
