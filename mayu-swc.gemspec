# frozen_string_literal: true

require_relative "lib/mayu/swc/version"

Gem::Specification.new do |spec|
  spec.name = "mayu-swc"
  spec.version = Mayu::SWC::VERSION
  spec.authors = ["Andreas Alin"]
  spec.email = ["andreas.alin@gmail.com"]

  spec.summary = "JS/TS transformer for Mayu"
  # spec.description = "TODO: Write a longer description or delete this line."
  spec.homepage = "https://github.com/mayu-live/mayu-swc"
  spec.license = "MPL"
  spec.required_ruby_version = ">= 3.2.0"
  spec.required_rubygems_version = ">= 3.3.11"

  # spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  # spec.metadata["source_code_uri"] = "TODO: Put your gem's public repo URL here."
  # spec.metadata["changelog_uri"] = "TODO: Put your gem's CHANGELOG.md URL here."

  spec.platform = Gem::Platform::RUBY
  spec.files = Dir["lib/**/*.rb"].concat(Dir["ext/src/**/*.rs"]) << "ext/Cargo.toml" << "Cargo.toml" << "Cargo.lock" << "README.md"
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/Cargo.toml"]

  spec.add_development_dependency "rake-compiler", "~> 1.2.5"
  spec.add_development_dependency "rb_sys", "~> 0.9.86"
  spec.add_development_dependency "rake", "~> 13.1"
  spec.add_development_dependency "minitest", "~> 5.20"
end
