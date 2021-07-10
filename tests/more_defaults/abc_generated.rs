// automatically generated by the FlatBuffers compiler, do not modify
use std::mem;
use std::cmp::Ordering;
extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ABC: i32 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ABC: i32 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ABC: [ABC; 3] = [
  ABC::A,
  ABC::B,
  ABC::C,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ABC(pub i32);
#[allow(non_upper_case_globals)]
impl ABC {
  pub const A: Self = Self(0);
  pub const B: Self = Self(1);
  pub const C: Self = Self(2);

  pub const ENUM_MIN: i32 = 0;
  pub const ENUM_MAX: i32 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::A,
    Self::B,
    Self::C,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::A => Some("A"),
      Self::B => Some("B"),
      Self::C => Some("C"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for ABC {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ABC {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<i32>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for ABC {
    type Output = ABC;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<i32>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for ABC {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i32::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = i32::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ABC {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i32::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ABC {}
