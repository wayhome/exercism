class Gigasecond
    def self.from(since : Time) Time
        since + 1e9.seconds
    end
end
