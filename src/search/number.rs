
use std::ops::*;
use std::fmt::Display;

// The `Number` trait. Used in trait bounds by Expression and Searcher. If you implement this trait for your own type, you can use that type with these structs.

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
{
    fn from_u8(from: u8) -> Self;
    fn min() -> Self;
    fn is_signed() -> bool;
}

impl Number for u8    {fn from_u8(from: u8) -> Self {from}         fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}
impl Number for u16   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}
impl Number for u32   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}
impl Number for u64   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}
impl Number for u128  {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}
impl Number for usize {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {false}}

impl Number for i8    {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}
impl Number for i16   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}
impl Number for i32   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}
impl Number for i64   {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}
impl Number for i128  {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}
impl Number for isize {fn from_u8(from: u8) -> Self {from as Self} fn min() -> Self {Self::MIN} fn is_signed() -> bool {true}}

