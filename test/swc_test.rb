# frozen_string_literal: true

require "minitest/autorun"

require_relative "../lib/mayu/swc"

class Mayu::SWC::Test < Minitest::Test
  def test_transform
    transformed = Mayu::SWC.transform("#{__method__}.ts", <<~TypeScript)
      export default function Hello(asd: string) {
        console.log(asd)
      }
    TypeScript

    assert_equal(
      transformed,
      'export default function Hello(asd){console.log(asd);}'
    )
  end
end
