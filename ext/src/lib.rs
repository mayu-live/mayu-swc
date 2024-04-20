use magnus::{
    class, define_module, exception::ExceptionClass, function, gc::register_mark_object, method,
    prelude::*, value::Lazy, Error, RModule, Ruby,
};

use swc_common::{
    self,
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

use std::{collections::HashMap, convert::Infallible};

fn transform(ruby: &Ruby, filename: String, source: String) -> Result<String, Error> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let comments = SingleThreadedComments::default();

    let fm = cm
        .new_source_file_from(swc_common::FileName::Custom(filename.clone()), source.into());

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: filename.ends_with(".tsx"),
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let program = parser
        .parse_program()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parse program.");

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // Optionally transforms decorators here before the resolver pass
        // as it might produce runtime declarations.

        // Conduct identifier scope analysis
        let program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        let program = program.fold_with(&mut strip(top_level_mark));

        // Fix up any identifiers with the same name, but different contexts
        let program = program.fold_with(&mut hygiene());

        // Ensure that we have enough parenthesis.
        let program = program.fold_with(&mut fixer(Some(&comments)));

        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: swc_ecma_codegen::Config::default().with_minify(true),
                cm: cm.clone(),
                comments: Some(&comments),
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_program(&program).unwrap();
        }

        Ok(String::from_utf8(buf).expect("non-utf8?"))
    })
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let mayu = define_module("Mayu")?;
    let module = mayu.define_module("SWC")?;
    module.define_singleton_method("ext_transform", function!(transform, 2))?;

    Ok(())
}
