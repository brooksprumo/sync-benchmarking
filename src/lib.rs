pub const SIZE_OF_SMALL: usize = 32;
pub const SIZE_OF_LARGE: usize = 1024;

const SIZE_OF_COUNTER: usize = std::mem::size_of::<u64>();
const SIZE_OF_SMALL_BUFFER: usize = SIZE_OF_SMALL - SIZE_OF_COUNTER;
const SIZE_OF_LARGE_BUFFER: usize = SIZE_OF_LARGE - SIZE_OF_COUNTER;

/// A generic type that is "small".  Larger than a single register,
/// but small enough to fit into a cache line.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Small {
    pub counter: u64,
    _buffer: [u8; SIZE_OF_SMALL_BUFFER],
}

/// A generic type that is "large".  Larger than a cache line.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Large {
    pub counter: u64,
    _buffer: [u8; SIZE_OF_LARGE_BUFFER],
}

impl Default for Small {
    fn default() -> Self {
        Self {
            counter: 0,
            _buffer: [0; SIZE_OF_SMALL_BUFFER],
        }
    }
}

impl Default for Large {
    fn default() -> Self {
        Self {
            counter: 0,
            _buffer: [0; SIZE_OF_LARGE_BUFFER],
        }
    }
}

pub trait Counter {
    fn counter(&self) -> u64;
    fn counter_mut(&mut self) -> &mut u64;
}

impl Counter for Small {
    fn counter(&self) -> u64 {
        self.counter
    }
    fn counter_mut(&mut self) -> &mut u64 {
        &mut self.counter
    }
}

impl Counter for Large {
    fn counter(&self) -> u64 {
        self.counter
    }
    fn counter_mut(&mut self) -> &mut u64 {
        &mut self.counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
     * use proptest::prelude::*;
     */

    #[test]
    fn test_size_of_small() {
        assert_eq!(std::mem::size_of::<Small>(), SIZE_OF_SMALL);
    }

    #[test]
    fn test_size_of_large() {
        assert_eq!(std::mem::size_of::<Large>(), SIZE_OF_LARGE);
    }

    /*
     *     proptest! {
     *         #[test]
     *         fn pbt_log2_ceil_v1(x in any::<u64>()) {
     *             prop_assert_eq!(
     *                 log2_ceil_baseline(x),
     *                 log2_ceil_v1(x)
     *             );
     *         }
     *
     *         #[test]
     *         fn pbt_log2_ceil_v2(x in any::<u64>()) {
     *             if let Some(x) = NonZeroU64::new(x) {
     *                 prop_assert_eq!(
     *                     log2_ceil_baseline(x.get()),
     *                     log2_ceil_v2(x)
     *                 );
     *             }
     *         }
     *
     *         #[test]
     *         fn pbt_log2_ceil_v3(x in any::<u64>()) {
     *             if let Some(x) = NonZeroU63::new(x) {
     *                 prop_assert_eq!(
     *                     log2_ceil_baseline(x.get()),
     *                     log2_ceil_v3(x)
     *                 );
     *             }
     *         }
     *     }
     */
}
