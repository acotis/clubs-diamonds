
use std::ops::*;
use num_traits::ops::{checked::*, wrapping::*};

/// Used in trait bounds by [`Expression`][crate::Expression] and [`Searcher`][crate::Searcher]. Exposed as part of the public ABI so that you can use it in trait bounds for functions that take or return [`Expression`][crate::Expression]s.
///
/// **Note:** This trait is even less stable than the rest of the crate. Do not implement it for your type unless you're okay with it changing later and ruining your code.

pub trait Number: 
    Copy + Send + 'static
    + Not<Output = Self>
    + WrappingMul
    + CheckedDiv
    + CheckedRem
    + WrappingAdd
    + WrappingSub
    + WrappingShl
    + WrappingShr
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOr<Output = Self>
{

    /// Convert a `u8` into a `Self`. Used by [`Expression`][crate::Expression] when a constant appears in an expression (because internally constants are stored as `u8`s and they need to be converted into the [`Expression`][crate::Expression]'s variable type to do math with them.

    fn from_u8(from: u8) -> Self;

    /// Return `true` if the type is signed and `false` if it's unsigned.

    fn is_signed() -> bool;

    /// Return the maximum allowable value for this type as a `u128`. Used by [`Searcher`] to limit the constant values it considers in expressions.

    fn max_as_u128() -> u128;

    /// Convert a `Self` into a `u32`. Used by [`Expression`][crate::Expression] when calling the `.wrapping_shl(self, rhs: u32)` function, which is the tool used to invoke the release-mode behavior of the `<<` even from debug mode, but which for some reason accepts a `u32` as the right-hand side instead of any numeric type.

    fn as_u32(self) -> u32;
}

impl Number for u8    {fn from_u8(from: u8) -> Self {from}         fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for u16   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for u32   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for u64   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for u128  {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for usize {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}

impl Number for i8    {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for i16   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for i32   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for i64   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for i128  {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}
impl Number for isize {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32} fn max_as_u128() -> u128 {Self::MAX as u128}}

