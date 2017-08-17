// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#pragma once

#include <stdint.h>

// rust ffi

// string
struct rust_string;

// string_ptr
struct rust_string_ptr {
	const uint8_t* ptr;
	size_t len;
};

// return ptr to rust_str
struct rust_string_ptr* rust_string_ptr(const struct rust_string* s);

// removes rust string
void rust_string_destroy(struct rust_string* s);

// removes string pointer
void rust_string_ptr_destroy(struct rust_string_ptr* s);

struct rust_string* hello_world(struct rust_string_ptr* s);
