use murmur3::murmur3_32;

// Probability of false positives
#[derive(Clone, Copy)]
pub struct Probability {
    pub value: f32,
}

impl Default for Probability {
    fn default() -> Self {
        Self { value: 0.2 }
    }
}

// Results of the filter
pub struct FilterResults {
    bitset: Vec<bool>,
    number_of_hash_functions: u8,
    bit_array_size: usize,
    probability: f32,
}

impl FilterResults {
    pub fn new(
        bitset: Vec<bool>,
        number_of_hash_functions: u8,
        bit_array_size: usize,
        probability: f32,
    ) -> Self {
        FilterResults {
            bitset,
            number_of_hash_functions,
            bit_array_size,
            probability,
        }
    }

    pub fn get_number_of_hash_functions(&self) -> [u8; 2] {
        (self.number_of_hash_functions as u16).to_be_bytes()
    }

    pub fn get_array_size(&self) -> [u8; 4] {
        (self.bit_array_size as u32).to_be_bytes()
    }

    pub fn get_probability(&self) -> [u8; 4] {
        self.probability.to_be_bytes()
    }

    pub fn get_bitset(&self) -> Vec<u8> {
        let mut bitset = Vec::new();
        for bit in &self.bitset {
            bitset.push(if *bit { 1 } else { 0 });
        }
        bitset
    }
}

/// Bloom filter, using murmur3 hash function
///
/// Example:
/// - m = 2^24 = 16_777_216 <- Maximum size of the bloom filter
/// - n = 380_000 <- Expected number of elements to be inserted
/// - k = 10 <- Number of hash functions
/// - P = 1.17 x 10^-7 <- Calculation of the probability of false positives (1 - [1 - 1/m]^(kn))^k
pub struct BloomFilter {
    bitset: Vec<bool>,
    number_of_hashes: u8,
    item_count: usize,
    probability_of_false_positives: f32,
}

impl BloomFilter {
    /// Create a new bloom filter
    pub fn new(
        item_count: usize,
        expected_probability: Option<Probability>,
        expected_size: Option<usize>,
        expected_number_of_hashes: Option<u8>,
    ) -> Self {
        let probability = expected_probability.unwrap_or_default();
        let size = expected_size.unwrap_or(Self::calculate_size(item_count, probability.value));
        let number_of_hashes =
            expected_number_of_hashes.unwrap_or(Self::calculate_hash_count(size, item_count));

        BloomFilter {
            bitset: vec![false; size],
            number_of_hashes,
            item_count,
            probability_of_false_positives: probability.value,
        }
    }

    /// Insert a value into the bloom filter
    pub fn insert(&mut self, value: &mut &[u8]) {
        for iteration in 0..self.number_of_hashes {
            let hashed_value = murmur3_32(value, iteration as u32).unwrap_or_default();
            let index = (hashed_value as usize) % self.bitset.len();
            self.bitset[index] = true;
        }
    }

    /// Check if a value exists in the bloom filter
    pub fn lookup(&self, value: &mut &[u8]) -> bool {
        for iteration in 0..self.number_of_hashes {
            let hashed_value = murmur3_32(value, iteration as u32).unwrap_or_default();
            let index = (hashed_value as usize) % self.bitset.len();
            if !self.bitset[index] {
                return false;
            }
        }
        true
    }

    pub fn count(&self) -> usize {
        self.item_count
    }

    pub fn get_probability(&self) -> f32 {
        self.probability_of_false_positives
    }

    // Return the results of the filter
    pub fn get_result(&self) -> FilterResults {
        FilterResults {
            bitset: self.bitset.clone(),
            number_of_hash_functions: self.number_of_hashes,
            bit_array_size: self.bitset.len(),
            probability: self.probability_of_false_positives,
        }
    }

    /// Return the size of bit array(m) to used using the following formula:
    ///
    /// `m = - ((n * ln(p))/(ln(2)^2))`
    ///
    /// where:
    /// - m = size of bit array
    /// - n = number of elements expected to be inserted
    /// - p = false positive probability
    fn calculate_size(number_of_elements: usize, false_positive_probability: f32) -> usize {
        let n = number_of_elements;
        let p = false_positive_probability;
        let m = -((n as f32) * p.ln()) / (2.0f32.ln().powi(2));

        m.ceil() as usize
    }

    /// Return the number of hash functions(k) to be used using the following formula:
    ///
    /// `k = (m/n) * ln(2)`
    ///
    /// where:
    /// - k = number of hash functions
    /// - m = size of bit array
    /// - n = number of elements expected to be inserted
    fn calculate_hash_count(size_of_array: usize, number_of_elements: usize) -> u8 {
        let m = size_of_array;
        let n = number_of_elements;
        let k = (m as f32 / n as f32) * 2.0f32.ln();

        k.ceil() as u8
    }
}

impl From<FilterResults> for BloomFilter {
    fn from(results: FilterResults) -> Self {
        BloomFilter {
            bitset: results.bitset,
            number_of_hashes: results.number_of_hash_functions,
            item_count: results.bit_array_size,
            probability_of_false_positives: results.probability,
        }
    }
}

#[cfg(test)]
mod bloom_filter_tests {
    use super::*;

    #[test]
    fn test_bloom_filter_insert() {
        let mut bloom_filter = BloomFilter::new(100, None, None, None);
        let mut word = "hello".as_bytes();
        bloom_filter.insert(&mut word);
        let mut word = "hello".as_bytes();
        assert!(bloom_filter.lookup(&mut word));
    }

    #[test]
    fn test_bloom_filter_lookup() {
        let bloom_filter = BloomFilter::new(100, None, None, None);
        let mut word = "hello".as_bytes();
        assert!(!bloom_filter.lookup(&mut word));
    }

    #[test]
    fn test_bloom_filter_calculate_size() {
        let number_of_elements = 100;
        let false_positive_probability = 0.2;
        let size = BloomFilter::calculate_size(number_of_elements, false_positive_probability);
        assert_eq!(size, 335);
    }

    #[test]
    fn test_bloom_filter_calculate_hash_count() {
        let size_of_array = 335;
        let number_of_elements = 100;
        let hash_count = BloomFilter::calculate_hash_count(size_of_array, number_of_elements);
        assert_eq!(hash_count, 3);
    }
}
