# Please implement your solution to raindrops in this file
class Raindrops
    def self.drops(num : Int) String
        output = ""
        output += "Pling" if num.divisible_by?(3)
        output += "Plang" if num.divisible_by?(5)
        output += "Plong" if num.divisible_by?(7)
        output.blank? ? num.to_s : output 
    end
end