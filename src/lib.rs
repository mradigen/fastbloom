use napi_derive::napi;
use napi::bindgen_prelude::*;
use fastbloom::AtomicBloomFilter as FastBloom;
use rayon::prelude::*;

#[napi]
pub struct BloomFilter {
    filter: FastBloom,
}

#[napi]
impl BloomFilter {

    /// Creates a new Bloom Filter with the specified capacity and false positive rate.
    /// @param capacity - The expected number of items to be stored in the Bloom Filter.
    /// @param false_positive_rate - The desired false positive rate (between 0 and 1).
    /// @throws Will throw an error if the capacity is not greater than 0 or if the false positive rate is not between 0 and 1.
    /// @returns A new instance of the Bloom Filter.
    #[napi(constructor)]
    pub fn new(capacity: f64, false_positive_rate: f64) -> Result<Self> {
        if capacity == 0.0 {
            return Err(Error::from_reason("Capacity must be greater than 0"));
        }
        if false_positive_rate <= 0.0 || false_positive_rate >= 1.0 {
            return Err(Error::from_reason("False positive rate must be between 0 and 1"));
        }

        let filter = FastBloom::with_false_pos(false_positive_rate)
            .expected_items(capacity as usize);

        Ok(BloomFilter {
            filter,
        })
    }

    /// Adds an item to the Bloom Filter. This method is thread-safe and can be called concurrently from multiple threads.
    /// @param item - The item to be added to the Bloom Filter.
    /// @returns void
    #[napi]
    pub fn add(&self, item: String) {
        self.filter.insert(&item);
    }

    /// Checks if an item is possibly in the Bloom Filter. This method is thread-safe and can be called concurrently from multiple threads.
    /// @param item - The item to be checked for membership in the Bloom Filter.
    /// @returns A boolean indicating whether the item is possibly in the Bloom Filter (true) or definitely not in the Bloom Filter (false).
    #[napi]
    pub fn has(&self, item: String) -> bool {
        self.filter.contains(&item)
    }

    /// Adds multiple items to the Bloom Filter in parallel. This method is thread-safe and can be called concurrently from multiple threads.
    /// @param items - An array of items to be added to the Bloom Filter.
    /// @returns void
    #[napi]
    pub fn bulk_add(&self, items: Vec<String>) {
        items.par_iter().for_each(|item| {
            self.filter.insert(item);
        });
    }
}
