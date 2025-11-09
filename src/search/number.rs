
use std::ops::*;
use num_traits::ops::wrapping::*;
use std::fmt::Display;

/// Used in trait bounds by `Expression` and `Searcher`. Exposed as part of the public ABI so that you can use it in trait bounds for functions that take or return `Expression`s.

pub trait Number: 
    Copy + PartialEq + Eq 
    + PartialOrd + Ord + Send + 'static
    + Default + Display
    + Not<Output = Self>
    + Mul<Output = Self>    + MulAssign
    + Div<Output = Self>    + DivAssign
    + Rem<Output = Self>    + RemAssign
    + Add<Output = Self>    + AddAssign
    + Sub<Output = Self>    + SubAssign
    + Shl<Output = Self>    + ShlAssign
    + Shr<Output = Self>    + ShrAssign
    + BitAnd<Output = Self> + BitAndAssign
    + BitXor<Output = Self> + BitXorAssign
    + BitOr<Output = Self>  + BitOrAssign

    + WrappingShl
    + WrappingShr
    + WrappingAdd
    + WrappingSub
    + WrappingMul
{

    /// Convert a `u8` into a `Self`. Used by `Expression` when a constant appears in an expression (because internally constants are stored as `u8`s and they need to be converted into the `Expression`'s variable type to do math with them.

    fn from_u8(from: u8) -> Self;

    /// Return the minimum value representable by this type. Used by `Expression` to detect a crash-case where, for signed types, dividing or modulo-ing the minimum representable value by `-1` is a runtime error.

    fn min() -> Self;

    /// Return `true` if the type is signed and `false` if it's unsigned. Used by `Expression` in conjunction with `Number`'s `.min()` method.

    fn is_signed() -> bool;

    /// Convert a `Self` into a `u32`. Used by `Expression` when calling the `.wrapping_shl(self, rhs: u32)` function, which is the tool used to invoke the release-mode behavior of the `<<` even from debug mode, but which for some reason accepts a `u32` as the right-hand side instead of any numeric type.

    fn as_u32(self) -> u32;
}

impl Number for u8    {fn from_u8(from: u8) -> Self {from}         fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u16   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u32   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u64   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u128  {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for usize {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}

impl Number for i8    {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i16   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i32   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i64   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i128  {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for isize {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}

