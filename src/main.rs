use arboard::Clipboard;
use jaq_core::load::{Arena, File, Loader};
use jaq_core::{Compiler, Ctx, RcIter};
use jaq_json::Val;
use serde_json::Value;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Get clipboard content
    let mut clipboard = Clipboard::new()?;
    let input = clipboard.get_text()?;

    if args.len() >= 2 {
        // jq filter mode
        run_query(&input, &args[1])
    } else {
        // Pretty-print mode
        let parsed: Value = match serde_json::from_str(&input) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Похоже, что в буфере обмена не JSON ¯\\_(ツ)_/¯");
                return Ok(());
            }
        };
        println!("{}", serde_json::to_string_pretty(&parsed)?);
        Ok(())
    }
}

fn run_query(input: &str, filter: &str) -> Result<(), Box<dyn Error>> {
    let parsed: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Похоже, что в буфере обмена не JSON ¯\\_(ツ)_/¯");
            return Ok(());
        }
    };

    let program = File {
        code: filter,
        path: (),
    };

    let loader = Loader::new(jaq_std::defs().chain(jaq_json::defs()));
    let arena = Arena::default();

    let modules = loader.load(&arena, program).map_err(|errs| {
        format!(
            "Ошибка разбора фильтра: {}",
            errs.into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;

    let compiled = Compiler::default()
        .with_funs(jaq_std::funs().chain(jaq_json::funs()))
        .compile(modules)
        .map_err(|errs| {
            format!(
                "Ошибка компиляции фильтра: {}",
                errs.into_iter()
                    .map(|e| format!("{e:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    let inputs = RcIter::new(core::iter::empty());
    let out = compiled.run((Ctx::new([], &inputs), Val::from(parsed)));

    for result in out {
        match result {
            Ok(val) => println!("{}", serde_json::to_string_pretty(&Value::from(val))?),
            Err(e) => eprintln!("Ошибка: {e:?}"),
        }
    }

    Ok(())
}
