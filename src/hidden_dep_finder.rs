// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

const STD_CONSTRUCTORS: &[&str] = &[
    "Box", "Arc", "Rc", "String", "Vec", "HashMap", "HashSet",
    "Option", "Result", "Ok", "Err", "Some", "None",
    "Ordering", "Duration", "Path", "PathBuf", "CString", "CStr",
    "OsString", "OsStr", "Default", "Clone",
];

const STD_MODULE_CALLS: &[&str] = &[
    "fs::read", "fs::write", "fs::read_to_string", "fs::read_dir",
    "fs::create_dir", "fs::create_dir_all", "fs::remove_file",
    "fs::remove_dir", "fs::remove_dir_all", "fs::copy", "fs::rename",
    "fs::metadata", "env::var", "env::args", "env::temp_dir",
    "env::current_dir", "env::current_exe", "env::set_var",
    "process::exit", "process::abort", "process::id",
    "thread::sleep", "thread::spawn",
    "net::TcpStream", "net::TcpListener", "net::UdpSocket",
];

pub(crate) struct HiddenDepFinder {
    pub(crate) count: usize,
    concrete_fields: Vec<String>,
}

impl HiddenDepFinder {
    pub(crate) fn new() -> Self {
        Self {
            count: 0,
            concrete_fields: Vec::new(),
        }
    }

    pub(crate) fn set_concrete_fields(&mut self, fields: Vec<String>) {
        self.concrete_fields = fields;
    }

    fn check_path(&mut self, path: &syn::Path) {
        let segments: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        if segments.is_empty() {
            return;
        }

        let tail_start = segments.len().saturating_sub(2);
        let tail = segments[tail_start..].join("::");

        if STD_MODULE_CALLS.contains(&tail.as_str())
            && (segments.len() <= 2 || segments[0] == "std" || segments[0] == "core")
        {
            self.count += 1;
            return;
        }

        let first = &segments[0];
        if first == "Self" || first == "self" {
            return;
        }
        if first.starts_with(|c: char| c.is_ascii_uppercase())
            && !STD_CONSTRUCTORS.contains(&first.as_str())
        {
            self.count += 1;
        }
    }
}

fn is_print_macro(path: &syn::Path) -> bool {
    if let Some(name) = path.get_ident() {
        let n = name.to_string();
        return n == "println" || n == "eprintln" || n == "print" || n == "eprint";
    }
    false
}

impl<'ast> Visit<'ast> for HiddenDepFinder {
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        if let syn::Stmt::Macro(stmt_macro) = stmt {
            if is_print_macro(&stmt_macro.mac.path) {
                self.count += 1;
                return;
            }
        }
        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        match expr {
            syn::Expr::Call(expr_call) => {
                if let syn::Expr::Path(expr_path) = &*expr_call.func {
                    self.check_path(&expr_path.path);
                }
            }
            syn::Expr::MethodCall(expr_method) => {
                if let syn::Expr::Path(expr_path) = &*expr_method.receiver {
                    if expr_path.path.segments.len() == 1
                        && expr_path.path.segments[0].ident == "self"
                    {
                        return;
                    }
                }
                if let syn::Expr::Field(expr_field) = &*expr_method.receiver {
                    if let syn::Expr::Path(expr_path) = &*expr_field.base {
                        if expr_path.path.segments.len() == 1
                            && expr_path.path.segments[0].ident == "self"
                        {
                            if let syn::Member::Named(ident) = &expr_field.member {
                                let name = ident.to_string();
                                if self.concrete_fields.contains(&name) {
                                    self.count += 1;
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            syn::Expr::Macro(expr_macro) => {
                if is_print_macro(&expr_macro.mac.path) {
                    self.count += 1;
                    return;
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }

    fn visit_expr_unsafe(&mut self, _expr: &'ast syn::ExprUnsafe) {
        self.count += 1;
    }
}
