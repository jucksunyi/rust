// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(staged_api)]
#![doc(issue_tracker_base_url = "http://issue_url/")]

#![unstable(feature="test", issue = "32374")]

// @has issue_32374/index.html '//*[@class="docblock short"]' \
//      '[Deprecated] [Unstable]'

// @has issue_32374/struct.T.html '//*[@class="stab deprecated"]' \
//      'Deprecated since 1.0.0: text'
// @has - '<code>test</code>'
// @has - '<a href="http://issue_url/32374">#32374</a>'
#[rustc_deprecated(since = "1.0.0", reason = "text")]
#[unstable(feature = "test", issue = "32374")]
pub struct T;
