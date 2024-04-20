# Mayu::SWC

This library transforms/minfies TypeScript and JavaScript.

It uses [swc](https://github.com/swc-project/swc) ([webpage](https://swc.rs/)),
which takes JavaScript / TypeScript files using modern JavaScript features and
outputs valid code that is supported by all major browsers.

Disclaimer: I do not know Rust. I have no idea what I'm doing.

## Installation

Mayu::SWC is [published on rubygems.org](https://rubygems.org/gems/mayu-swc)
and there are pre-built extensions for:

* `aarch64-linux`
* `arm64-darwin`
* `x86_64-darwin`
* `x86_64-linux`

```
gem install mayu-swc
```

## Usage

```ruby
require "mayu/swc"

code = Mayu::SWC.transform("/app/components/Hello.ts", <<~TypeScript)
  export default function hello(foo: string): string {
    console.log(foo)
    return "bar"
  }
TypeScript

puts code
```

Output:

```js
export default function hello(foo){console.log(foo);return"bar";}
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/mayu-live/swc.

## License

MIT
