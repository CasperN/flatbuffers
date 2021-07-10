// automatically generated by the FlatBuffers compiler, do not modify
use std::mem;
use std::cmp::Ordering;
extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum MonsterOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
    type Inner = Monster<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Monster<'a> {
    pub const fn get_fully_qualified_name() -> &'static str {
        "MyGame.Example2.Monster"
    }

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Monster { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        _args: &'args MonsterArgs) -> flatbuffers::WIPOffset<Monster<'bldr>> {
      let mut builder = MonsterBuilder::new(_fbb);
      builder.finish()
    }

    pub fn unpack(&self) -> MonsterT {
      MonsterT {
      }
    }
}

impl flatbuffers::Verifiable for Monster<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct MonsterArgs {
}
impl<'a> Default for MonsterArgs {
    #[inline]
    fn default() -> Self {
        MonsterArgs {
        }
    }
}
pub struct MonsterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Monster<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Monster");
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MonsterT {
}
impl Default for MonsterT {
  fn default() -> Self {
    Self {
    }
  }
}
impl MonsterT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Monster<'b>> {
    Monster::create(_fbb, &MonsterArgs{
    })
  }
}
