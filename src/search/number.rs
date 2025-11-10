
use std::ops::*;
use num_traits::ops::{checked::*, wrapping::*};

/// Used in trait bounds by `Expression` and `Searcher`. Exposed as part of the public ABI so that you can use it in trait bounds for functions that take or return `Expression`s.

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

    /// Convert a `u8` into a `Self`. Used by `Expression` when a constant appears in an expression (because internally constants are stored as `u8`s and they need to be converted into the `Expression`'s variable type to do math with them.

    fn from_u8(from: u8) -> Self;

    /// Return `true` if the type is signed and `false` if it's unsigned. Used by `Expression` in conjunction with `Number`'s `.min()` method.

    fn is_signed() -> bool;

    /// Convert a `Self` into a `u32`. Used by `Expression` when calling the `.wrapping_shl(self, rhs: u32)` function, which is the tool used to invoke the release-mode behavior of the `<<` even from debug mode, but which for some reason accepts a `u32` as the right-hand side instead of any numeric type.

    fn as_u32(self) -> u32;
}

impl Number for u8    {fn from_u8(from: u8) -> Self {from}         fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u16   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u32   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u64   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for u128  {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}
impl Number for usize {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {false} fn as_u32(self) -> u32 {self as u32}}

impl Number for i8    {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i16   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i32   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i64   {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for i128  {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}
impl Number for isize {fn from_u8(from: u8) -> Self {from as Self} fn is_signed() -> bool {true}  fn as_u32(self) -> u32 {self as u32}}

