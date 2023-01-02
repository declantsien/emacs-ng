// Copyright 2019-2020 the Deno authors. All rights reserved. MIT license.
use rusty_v8 as v8;

pub fn main() {
  let mut isolate = v8::Isolate::new(mock());
  let mut _scope = v8::EscapableHandleScope::new(&mut isolate);
}

fn mock<T>() -> T {
  unimplemented!()
}
