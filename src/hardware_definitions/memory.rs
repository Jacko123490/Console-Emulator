use std::error::Error;
use std::fmt::{Debug, Display, Formatter, LowerHex};
use std::iter::Sum;
use std::mem;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::slice;
use num_traits::{Unsigned};

#[derive(Debug)]
pub enum MemoryError {
    AddressRangeError(String),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            MemoryError::AddressRangeError(msg) => { write!(f, "{msg}") },
        }
    }
}

impl Error for MemoryError {}

// MemoryAddress
// (Wrapper for uint types that could be used for memory addressing)
// ----------------------------------------------------------------------------
pub trait MemAddr:
Sized
+ Copy
+ Eq
+ Ord
+ Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Self, Output = Self>
+ Div<Self, Output = Self>
+ Rem<Self, Output = Self>
+ Sum
+ AddAssign<Self>
+ SubAssign<Self>
+ MulAssign<Self>
+ DivAssign<Self>
+ RemAssign<Self>
+ Debug
+ Display
+ LowerHex
+ From<u8>  // Guaranteed conversion for each type
{}

impl MemAddr for u8 {}
impl MemAddr for u16 {}
impl MemAddr for u32 {}
impl MemAddr for u64 {}
impl MemAddr for u128 {}
impl MemAddr for usize {}

// AddressRange
// ----------------------------------------------------------------------------
/// A range of memory addresses within a larger address space.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AddressRange<AddressType: MemAddr> { // Ord + Display
    hi: AddressType,
    lo: AddressType,
}

impl<AddressType: MemAddr> AddressRange<AddressType> {
    pub fn new(mut lo: AddressType, mut hi: AddressType) -> AddressRange<AddressType> {
        if lo > hi {  mem::swap(&mut lo, &mut hi); }
        AddressRange { hi, lo }
    }

    /// The number of addresses covered by this range.
    pub fn len(&self) -> AddressType { self.hi - self.lo + AddressType::from(1) }

    /// Whether the provided address is within the AddressRange.
    pub fn in_range(&self, address: AddressType) -> bool {
        address >= self.lo && address <= self.hi
    }
}

impl<AddressType: MemAddr> Display for AddressRange<AddressType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:#x},{:#x}]", self.lo, self.hi)
    }
}

impl<AddressType: MemAddr> TryFrom<&[AddressType]> for AddressRange<AddressType> {
    type Error = MemoryError;

    fn try_from(value: &[AddressType]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Err(MemoryError::AddressRangeError("Need a value for address range, none provided in array.".to_string())),
            1 => Ok(AddressRange::new(value[0].clone(), value[0].clone())),  // Build single value address range
            _ => Ok(AddressRange::new(value[0].clone(), value[1].clone())),
        }
    }
}

// AddressSpace
// ----------------------------------------------------------------------------
/// A defined set of addresses within the global address space defined by 'AddressType'.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AddressSpace<AddressType: MemAddr> {
    address_space: Vec<AddressRange<AddressType>>,
}

impl<AddressType: MemAddr> AddressSpace<AddressType> {
    pub fn new(address_space: Vec<[AddressType; 2]>) -> AddressSpace<AddressType> {
        if address_space.len() == 0 { panic!("Cannot create an AddressSpace from an empty vector.")}

        // Convert to array first
        let mut address_space: Vec<AddressRange<AddressType>> = address_space.into_iter()
            .map(|x| x[..].try_into().expect("Invalid AddressRange entry in address space."))
            .collect();
        AddressSpace::collapse_space(&mut address_space);
        AddressSpace { address_space }
    }

    /// Collapse address ranges with overlapping elements into a single address range.
    fn collapse_space(address_space: &mut Vec<AddressRange<AddressType>>) {
        address_space.sort_by_key(|address| address.lo);

        let mut i = 0;
        while i < address_space.len() {
            let current_range = &address_space[i];
            let next_range = match address_space.get(i+1) {
                Some(next_range) => next_range,
                None => break,
            };
            // Combine overlapping or neighboring ranges
            if next_range.lo <= current_range.hi + AddressType::from(1) {
                address_space[i] = AddressRange::new(
                    current_range.lo,  // Due to sorting, should always be lowest bound of overlapping ranges
                    current_range.hi.max(next_range.hi)  // Get the larger bound of overlapping ranges
                );
                address_space.remove(i+1);  // Clear next element that we have merged with current one
            } else {
                i += 1;
            }
        }
    }

    // Utility Methods
    /// The number of addresses covered by this space.
    pub fn len(&self) -> AddressType { self.address_space.iter().map(|x| x.len()).sum() }

    /// Whether the provided address is within the AddressSpace.
    pub fn in_range(&self, address: AddressType) -> bool {
        for range in self.address_space.iter() {
            if range.in_range(address) { return true; }
        }
        false
    }

    /// Get the global bounds of the address space
    pub fn global_range(&self) -> AddressRange<AddressType> {
        AddressRange::new(
            self.address_space.first().expect("AddressSpace needs at least one element.").lo,
            self.address_space.last().expect("AddressSpace needs at least one element.").hi
        )
    }
}

impl<AddressType: MemAddr> Display for AddressSpace<AddressType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               self.address_space.iter()
                   .map(|x| x.to_string())
                   .collect::<Vec<String>>()
                   .join(",")
        )
    }
}

impl<AddressType: MemAddr> Index<usize> for AddressSpace<AddressType> {
    type Output = AddressRange<AddressType>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.address_space[index]
    }
}

impl<'a, AddressType: MemAddr> IntoIterator for &'a AddressSpace<AddressType> {
    type Item = &'a AddressRange<AddressType>;
    type IntoIter = slice::Iter<'a, AddressRange<AddressType>>;

    fn into_iter(self) -> Self::IntoIter {
        self.address_space.iter()
    }
}

// Memory Implementations
// ----------------------------------------------------------------------------
pub trait AddressableMemory<AddressType, DataType>:
    Index<AddressType, Output = DataType> +
    IndexMut<AddressType>
where
    AddressType: PrimInt + LowerHex,
    DataType: Unsigned,
{
    fn address_space(&self) -> &AddressSpace<AddressType>;
}

// Unit Tests
// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // AddressRange unit tests
    #[test]
    fn test_address_range_creation() {
        // Standard Range
        let test_range = AddressRange::new(1u8, 2u8);
        assert_eq!([test_range.lo, test_range.hi], [1u8, 2u8]);
        // Single Range
        let test_range = AddressRange::new(1u8, 1u8);
        assert_eq!([test_range.lo, test_range.hi], [1u8, 1u8]);
        // Inverted Range
        let test_range = AddressRange::new(2u8, 1u8);
        assert_eq!([test_range.lo, test_range.hi], [1u8, 2u8]);
    }

    #[test]
    fn test_address_range_slice_creation() {
        // Slice Double Range
        let array: [u8; 3] = [1u8, 2u8, 3u8];
        let test_range = AddressRange::try_from(&array[..]).expect("Invalid address range");
        assert_eq!([test_range.lo, test_range.hi], [1u8, 2u8]);
        // Slice Double Range
        let test_range = AddressRange::try_from(&array[0..2]).expect("Invalid address range");
        assert_eq!([test_range.lo, test_range.hi], [1u8, 2u8]);
        // Slice Single Range
        let test_range = AddressRange::try_from(&array[0..1]).expect("Invalid address range");
        assert_eq!([test_range.lo, test_range.hi], [1u8, 1u8]);
        // Empty Slice
        let test_range = AddressRange::try_from(<&[u8]>::default());
        assert!(test_range.is_err());
    }

    #[test]
    fn test_address_in_range() {
        // Standard Range
        let test_range = AddressRange::new(1u8, 5u8);
        assert!(!test_range.in_range(0u8));
        assert!(test_range.in_range(1u8));
        assert!(test_range.in_range(3u8));
        assert!(test_range.in_range(5u8));
        assert!(!test_range.in_range(6u8));
    }

    #[test]
    fn test_address_len() {
        let test_range = AddressRange::new(1u8, 5u8);
        assert_eq!(test_range.len(), 5);
    }

    // AdressSpace unit tests
    /// Ensure all range overlap types collapse correctly, array should be sorted and collapsed
    #[test]
    fn test_address_space_collapse() {
        let ranges: Vec<[u16; 2]> = vec![[0x0033, 0x0044], [0x0000, 0x0100], [0x0045, 0x0111], [0x0200, 0x02ff], [0x0300, 0x0311]];
        let addr_space = AddressSpace::new(ranges);
        let expected_ranges: Vec<AddressRange<u16>> = vec![
            AddressRange::new(0x0000, 0x0111),
            AddressRange::new(0x0200, 0x0311),
        ];
        assert_eq!(addr_space.address_space, expected_ranges);
    }

    #[test]
    fn test_address_space_in_range() {
        let ranges: Vec<[u16; 2]> = vec![[0x0001, 0x0100], [0x0200, 0x02ff]];
        let addr_space = AddressSpace::new(ranges);
        assert!(!addr_space.in_range(0x0000));
        assert!(addr_space.in_range(0x0001));
        assert!(addr_space.in_range(0x00f0));
        assert!(addr_space.in_range(0x0100));
        assert!(!addr_space.in_range(0x0101));
        assert!(addr_space.in_range(0x0200));
    }

    #[test]
    fn test_address_space_len() {
        let ranges: Vec<[u16; 2]> = vec![[0x0001, 0x0100], [0x0200, 0x02ff]];
        let addr_space = AddressSpace::new(ranges);
        assert_eq!(addr_space.len(), 256 + 256);
    }
}
