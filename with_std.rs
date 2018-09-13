// Copyright 2017, 2018 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[doc(hidden)]
pub mod imp {
	pub use std::cell::RefCell;
	pub use std::thread::LocalKey;
	pub use std::mem::transmute;
	pub use std::mem::replace;
	pub use std::marker::PhantomData;
}

#[doc(hidden)]
#[macro_export]
macro_rules! thread_local_impl {
	($(#[$attr:meta])* static $name:ident: $t:ty = $init:expr) => (
		thread_local!($(#[$attr])* static $name: $t = $init);
	);
}
