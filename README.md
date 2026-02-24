# fastbloom

A **blazingly fast** Bloom Filter for Node.js, implemented in Rust with native concurrency support.

**~40x faster** than the popular `bloom-filters` npm package, thanks to Rust's performance and parallel processing capabilities.

## Features

- **Exceptionally Fast**: Built on top of the `fastbloom` Rust crate with atomic operations, achieving ~40x speedup compared to `bloom-filters` npm package
- **Concurrent Operations**: Thread-safe implementation using `AtomicBloomFilter` - all operations can be called concurrently from multiple threads
- **Parallel Bulk Operations**: Uses `rayon` for parallel processing of bulk additions, maximizing multi-core CPU utilization
- **Optimal Sizing**: Automatically calculates the optimal bit array size and number of hash functions based on your desired capacity and false positive rate
- **N-API**: Uses N-API for stable Node.js ABI compatibility across Node.js versions
- **Zero-Cost Abstractions**: Direct Rust implementation with minimal JavaScript overhead

## Installation

```bash
npm install
```

## Build

You need to have Rust installed (`cargo`).

```bash
npm run build
```

This will compile the Rust code and generate the `index.js` binding file.

## Usage

```javascript
const { BloomFilter } = require('./index.js');

// 1. Initialize
// Capacity: 1,000,000 items
// False Positive Rate: 1% (0.01)
const filter = new BloomFilter(1000000, 0.01);

// 2. Add items (thread-safe)
filter.add('hello');
filter.add('world');

// 3. Check for existence (thread-safe)
console.log(filter.has('hello'));    // true
console.log(filter.has('universe')); // false

// 4. Bulk add (parallel processing for maximum performance)
filter.bulk_add(['apple', 'banana', 'cherry', 'date', 'elderberry']);
```

## API

### `new BloomFilter(capacity, false_positive_rate)`

Creates a new Bloom Filter with the specified parameters.

- **capacity** (`number`): The expected number of items to be stored (must be > 0)
- **false_positive_rate** (`number`): The desired false positive rate between 0 and 1 (e.g., 0.01 for 1%)

**Throws**: Error if capacity is not greater than 0 or if false positive rate is not between 0 and 1.

### `add(item)`

Adds an item to the Bloom Filter. **Thread-safe** - can be called concurrently.

- **item** (`string`): The item to add

### `has(item)`

Checks if an item is possibly in the Bloom Filter. **Thread-safe** - can be called concurrently.

- **item** (`string`): The item to check
- **Returns**: `boolean` - `true` if the item is possibly in the filter, `false` if definitely not

### `bulk_add(items)`

Adds multiple items to the Bloom Filter using **parallel processing**. **Thread-safe** - can be called concurrently.

- **items** (`string[]`): Array of items to add

This method uses `rayon` for parallel iteration, making it significantly faster than calling `add()` repeatedly for large datasets.