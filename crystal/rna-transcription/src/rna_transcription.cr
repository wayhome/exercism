class RnaComplement
    def self.of_dna(dna : String) String
        dna.tr("GCTA", "CGAU")
    end
end
