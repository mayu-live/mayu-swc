# frozen_string_literal: true

require "json"
require_relative "swc/version"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "mayu/swc/#{$1}/ext"
rescue LoadError
  require "mayu/swc/ext"
end

module Mayu
  module SWC
    def self.transform(filename, source)
      ext_transform(filename, source)
    end
  end
end
