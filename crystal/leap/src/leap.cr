# Please implement your solution to leap in this file
class Year
    def self.leap?(year : Int) Bool
	    (year%4 == 0 && year%100 != 0) || year%400 == 0
    end
end
