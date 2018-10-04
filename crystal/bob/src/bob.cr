class Bob
    def self.hey (words : String) String
        case
        when words == words.upcase && words != words.downcase
            "Whoa, chill out!"
        when words.blank?
            "Fine. Be that way!"
        when words.ends_with? '?'
            "Sure."
        else
            "Whatever."
        end
    end
end
