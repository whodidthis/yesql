#![crate_name = "yesql"]
#![crate_type = "dylib"]
#![feature(default_type_params)]
#![feature(phase)]
#![feature(globs)]
#![feature(plugin_registrar)]

extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::codemap;
use syntax::codemap::{Span};
use syntax::ext::base::*;
use syntax::ext::build::AstBuilder;
use syntax::ast;
use std::io::File;
use syntax::parse::token;
use regex::Regex;

pub fn parse<'a>(sql: &'a str, name: &str) -> Option<&'a str> {
    let regex_str = format!(r"(?is)--\s*name:\s*{}\s*(?P<fn>[^;]+)", name);
    let regex = Regex::new(regex_str.as_slice()).unwrap();
    match regex.captures(sql) {
        Some(caps) => Some(caps.name("fn")),
        None => return None,
    }
}

pub fn expand_sql_query(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
                           -> Box<MacResult + 'static> {
    let (file_expr, name_expr) = match get_exprs_from_tts(cx, sp, tts) {
        Some(ref exprs) if exprs.len() < 2 => {
            cx.span_err(sp, "no parameters given");
            return DummyResult::expr(sp);
        },
        None => return DummyResult::expr(sp),
        Some(exprs) => (exprs[0].clone(), exprs[1].clone()),
    };
    let file = match expr_to_string(cx, file_expr, "expected string literal") {
        None => return DummyResult::expr(sp),
        Some((v, _style)) => v,
    };
    let name = match expr_to_string(cx, name_expr, "expected string literal") {
        None => return DummyResult::expr(sp),
        Some((v, _style)) => v,
    };
    let file = res_rel_file(cx, sp, &Path::new(file));
    let bytes = match File::open(&file).read_to_end() {
        Err(e) => {
            cx.span_err(sp,
                        format!("couldn't read {}: {}",
                                file.display(),
                                e).as_slice());
            return DummyResult::expr(sp);
        },
        Ok(bytes) => bytes,
    };
    let src = match String::from_utf8(bytes) {
        Err(_) => {
            cx.span_err(sp,
                        format!("{} wasn't a utf-8 file",
                                file.display()).as_slice());
            return DummyResult::expr(sp);
        },
        Ok(src) => src,
    };
    match parse(src.as_slice(), name.get()) {
        None => {
            cx.span_err(sp,
                        format!("fn {} not found from file {}",
                                name.get(),
                                file.display()).as_slice());
            return DummyResult::expr(sp);
        },
        Some(f) => {
            let interned = token::intern_and_get_ident(f.as_slice());
            cx.codemap().new_filemap(file.display().to_string(), src.clone());
            MacExpr::new(cx.expr_str(sp, interned))
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("sql_query", expand_sql_query);
}

// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// // file at the top-level directory of this distribution and at
// // http://rust-lang.org/COPYRIGHT.
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.

// resolve a file-system path to an absolute file-system path (if it
// isn't already)
fn res_rel_file(cx: &mut ExtCtxt, sp: codemap::Span, arg: &Path) -> Path {
    // NB: relative paths are resolved relative to the compilation unit
    if !arg.is_absolute() {
        let mut cu = Path::new(cx.codemap().span_to_filename(sp));
        cu.pop();
        cu.push(arg);
        cu
    } else {
        arg.clone()
    }
}

