# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/extensiontask"

spec = Gem::Specification.load("mayu-swc.gemspec")
spec.requirements.clear
spec.required_ruby_version = nil
spec.required_rubygems_version = nil
spec.extensions.clear
spec.files -= Dir["ext/**/*"]

Rake::ExtensionTask.new("ext", spec) do |ext|
  ext.lib_dir = "lib/mayu/swc"
  ext.ext_dir = "ext"
  ext.cross_compile = true
  ext.cross_platform = [
    "x86_64-linux",
    "aarch64-linux",
    "x86_64-darwin",
    "arm64-darwin",
  ]
end

Gem::PackageTask.new(spec) do |pkg|
end

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--build"
end

require "minitest/test_task"

Minitest::TestTask.create(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.warning = false
  t.test_globs = ["test/**/*_test.rb"]
end

task default: :test
