#![feature(box_patterns)]

mod find_config;
mod read_file;
use find_config::find_config_path;
use std::{fs, path::PathBuf};
use swc_common::SourceMap;
use swc_common::{sync::Lrc, DUMMY_SP};
use swc_ecma_ast::{
    CallExpr, Callee, EmptyStmt, EsVersion, Expr, Ident, MemberExpr, MemberProp, MetaPropExpr,
    MetaPropKind, Module, Stmt,
};
use swc_ecma_codegen::text_writer::JsWriter;
use swc_ecma_codegen::Emitter;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{VisitMut, VisitMutWith};

fn main() {
    let source_map: Lrc<SourceMap> = Default::default();
    let mut buf = vec![];
    let file_path = find_config_path().expect("not found");
    let mut ast: Module = parse_module(file_path, source_map.clone());
    // 将源文件添加的sourcemap中
    let mut visitor = MyModule;
    let cfg = swc_ecma_codegen::Config::default()
        .with_minify(false)
        .with_target(EsVersion::Es5)
        .with_omit_last_semi(true)
        .with_ascii_only(false);
    ast.visit_mut_with(&mut visitor);
    {
        let writer = JsWriter::new(source_map.clone(), "\n", &mut buf, None);
        let mut emitter = Emitter {
            cfg,
            comments: None,
            cm: source_map,
            wr: Box::new(writer),
        };
        emitter.emit_module(&ast).expect("代码转换失败");
    }
    // 将 buffer 转换成字符串
    let code = String::from_utf8(buf).expect("Buffer 包含无效的 UTF-8");
    // 将代码写入到文件
    let output_file_path = "./output.js"; // 输出文件的路径
    fs::write(output_file_path, code).expect("文件写入失败");
}

fn parse_module(path: PathBuf, cm: Lrc<SourceMap>) -> Module {
    let fm = cm.load_file(&path).expect("没有找到文件");
    cm.new_source_file(
        swc_common::FileName::Real(path.clone()),
        fs::read_to_string(path.clone()).expect("文件读取失败"),
    );
    // 解析出来token
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    // 进行转化
    let mut parser = Parser::new_from(lexer);
    // 转换出来的ast
    let module = parser.parse_module().expect("转换失败");
    module
}

// 遍历ast,匹配特有的语法
struct MyModule;

impl VisitMut for MyModule {
    fn visit_mut_stmt(&mut self, stmt: &mut swc_ecma_ast::Stmt) {
        // 判断该语句是否包含成员表达式的调用
        match stmt {
            Stmt::Expr(expr_stmt) => {
                if is_import_meta_hot_call(&expr_stmt.expr) {
                    *stmt = Stmt::Empty(EmptyStmt { span: DUMMY_SP })
                }
            }
            _ => (),
        }
        stmt.visit_mut_children_with(self)
    }
}

fn is_import_meta_hot_call(expr: &Expr) -> bool {
    if let Expr::Call(CallExpr {
        callee: Callee::Expr(expr_callee),
        ..
    }) = expr
    {
        if let Expr::Member(expr_member) = &**expr_callee {
            if let MemberExpr {
                obj:
                    box Expr::Member(MemberExpr {
                        obj: box Expr::MetaProp(MetaPropExpr { kind, .. }),
                        prop: MemberProp::Ident(Ident { sym: hot_sym, .. }),
                        ..
                    }),
                ..
            } = expr_member
            {
                return *kind == MetaPropKind::ImportMeta && hot_sym == "hot";
            }
        }
    }
    false
}
