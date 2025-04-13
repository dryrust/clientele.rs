// This is free and unencumbered software released into the public domain.

use super::var;

pub fn ruby() -> Option<String> {
    var("RUBY")
}

/// See: https://linux.die.net/man/1/ruby
pub fn rubylib() -> Option<String> {
    var("RUBYLIB")
}

/// See: https://linux.die.net/man/1/ruby
pub fn rubypath() -> Option<String> {
    var("RUBYPATH")
}

/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler/man/bundle-config.1.ronn
pub fn bundle_cache_path() -> Option<String> {
    var("BUNDLE_CACHE_PATH")
}

/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler/man/bundle-config.1.ronn
/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler.rb
/// See: https://github.com/rubygems/rubygems/blob/master/bundler/spec/support/rubygems_ext.rb
/// See: https://github.com/rubygems/rubygems/blob/master/bundler/spec/install/path_spec.rb
pub fn bundle_path() -> Option<String> {
    var("BUNDLE_PATH")
}

/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler/gem_helper.rb
/// See: https://github.com/rubygems/rubygems/blob/master/bundler/spec/support/path.rb
pub fn gem() -> Option<String> {
    var("GEM").or_else(|| var("GEM_COMMAND"))
}

/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler.rb
/// See: https://github.com/rubygems/rubygems/blob/master/lib/rubygems/installer.rb
/// See: https://github.com/rubygems/rubygems/blob/master/lib/rubygems/path_support.rb
pub fn gem_home() -> Option<String> {
    var("GEM_HOME")
}

/// See: https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler.rb
/// See: https://github.com/rubygems/rubygems/blob/master/lib/rubygems/path_support.rb
pub fn gem_path() -> Option<String> {
    var("GEM_PATH")
}
