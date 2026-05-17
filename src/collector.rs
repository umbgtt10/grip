// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

use quote::ToTokens;
use syn::visit::Visit;
use syn::{Attribute, Item, ItemFn, Visibility};

use crate::function_info::FunctionInfo;
use crate::item_counts::ItemCounts;
use crate::unsafe_finder::UnsafeFinder;

fn self_ty_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .first()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

fn is_trait_object_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::TraitObject(_) => true,
        syn::Type::Reference(r) => is_trait_object_type(&r.elem),
        syn::Type::Path(p) => p.path.segments.iter().any(|seg| match &seg.arguments {
            syn::PathArguments::AngleBracketed(args) => args.args.iter().any(|arg| match arg {
                syn::GenericArgument::Type(t) => is_trait_object_type(t),
                _ => false,
            }),
            _ => false,
        }),
        _ => false,
    }
}

const KNOWN_FOREIGN_TRAITS: &[&str] = &[
    "Display", "Debug", "Clone", "Default", "PartialEq", "Eq",
    "PartialOrd", "Ord", "Hash", "Into", "From", "TryFrom",
    "Drop", "Deref", "DerefMut", "Index", "IndexMut",
    "Add", "Sub", "Mul", "Div", "Rem", "Neg", "Not",
    "Fn", "FnMut", "FnOnce", "Send", "Sync", "Sized",
    "ToString", "AsRef", "AsMut", "Borrow", "BorrowMut",
    "Error", "Read", "Write", "Seek", "BufRead",
    "Iterator", "IntoIterator", "Future", "IntoFuture",
    "Serialize", "Deserialize",
];

#[derive(Debug)]
pub struct Collector {
    counts: ItemCounts,
    functions: Vec<FunctionInfo>,
    current_file: String,
    struct_concrete_fields: HashMap<String, Vec<String>>,
}

impl Collector {
    fn new(file: String) -> Self {
        Self {
            counts: ItemCounts::default(),
            functions: Vec::new(),
            current_file: file,
            struct_concrete_fields: HashMap::new(),
        }
    }

    pub fn collect(source: &str, path: &Path) -> (ItemCounts, Vec<FunctionInfo>) {
        let file = path.to_string_lossy().replace('\\', "/");
        let syntax = match syn::parse_file(source) {
            Ok(s) => s,
            Err(_) => return (ItemCounts::default(), Vec::new()),
        };

        let mut collector = Self::new(file);
        for item in &syntax.items {
            collector.visit_item(item);
        }
        (collector.counts, collector.functions)
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(item_fn) if !self.has_test_attr(&item_fn.attrs) => self.visit_fn(item_fn),
            Item::Struct(item_struct) => self.visit_struct(item_struct),
            Item::Trait(item_trait) => self.visit_trait(item_trait),
            Item::Enum(item_enum) => self.visit_enum(item_enum),
            Item::Mod(item_mod) if !self.has_test_attr(&item_mod.attrs) => self.visit_mod(item_mod),
            Item::Impl(item_impl) => self.visit_impl(item_impl),
            _ => {}
        }
    }
}

impl Collector {
    fn visit_fn(&mut self, item_fn: &ItemFn) {
        let name = item_fn.sig.ident.to_string();
        let is_pub = matches!(
            self.classify_visibility(&item_fn.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        );
        let is_pure = self.is_probably_pure(item_fn);
        let hidden_deps = self.count_hidden_deps_in_block(&item_fn.block);
        let has_trait_seam = false;
        let contr = crate::contribution_schedule::contribution(is_pure, has_trait_seam, hidden_deps);

        self.counts.total_functions += 1;
        self.counts.total_items += 1;
        self.counts.total_contribution += contr;
        if contr == 1.0 {
            self.counts.clean_functions += 1;
        }
        match self.classify_visibility(&item_fn.vis) {
            VisibilityLevel::Pub => {
                self.counts.public_functions += 1;
                self.counts.public_items += 1;
            }
            VisibilityLevel::PubCrate => {
                self.counts.pubcrate_functions += 1;
                self.counts.public_items += 1;
            }
            _ => {}
        }
        if is_pure {
            self.counts.pure_functions += 1;
        }

        self.functions.push(FunctionInfo {
            name,
            file: self.current_file.clone(),
            is_pure,
            is_public: is_pub,
            hidden_deps,
            has_trait_seam,
        });
    }

    fn visit_struct(&mut self, item_struct: &syn::ItemStruct) {
        self.counts.total_items += 1;
        if matches!(
            self.classify_visibility(&item_struct.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_structs += 1;
            self.counts.public_items += 1;
        }
        let name = item_struct.ident.to_string();
        let concrete: Vec<String> = item_struct
            .fields
            .iter()
            .filter(|f| !is_trait_object_type(&f.ty))
            .map(|f| f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default())
            .filter(|n| !n.is_empty())
            .collect();
        self.struct_concrete_fields.insert(name, concrete);
    }

    fn visit_trait(&mut self, item_trait: &syn::ItemTrait) {
        self.counts.total_items += 1;
        if matches!(
            self.classify_visibility(&item_trait.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_traits += 1;
            self.counts.public_items += 1;
        }
    }

    fn visit_enum(&mut self, item_enum: &syn::ItemEnum) {
        self.counts.total_items += 1;
        if matches!(
            self.classify_visibility(&item_enum.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_enums += 1;
            self.counts.public_items += 1;
        }
    }

    fn visit_mod(&mut self, item_mod: &syn::ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }

    fn visit_impl(&mut self, item_impl: &syn::ItemImpl) {
        if item_impl.trait_.as_ref().is_some_and(|(_, p, _)| self.is_foreign_trait(p)) {
            return;
        }
        let is_trait_impl = item_impl.trait_.is_some();

        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                if self.has_test_attr(&method.attrs) {
                    continue;
                }
                let is_pure = !self.is_impl_method_impure(method);
                let self_ty_name = self::self_ty_name(&item_impl.self_ty);
                let concrete_fields = self.struct_concrete_fields.get(&self_ty_name).cloned().unwrap_or_default();
                let hidden_deps = self.count_hidden_deps_in_impl_method(method, concrete_fields);
                let has_trait_seam = is_trait_impl;
                let contr = crate::contribution_schedule::contribution(is_pure, has_trait_seam, hidden_deps);

                self.counts.total_functions += 1;
                self.counts.total_contribution += contr;
                if contr == 1.0 {
                    self.counts.clean_functions += 1;
                }
                if is_pure {
                    self.counts.pure_functions += 1;
                }

                let is_pub = matches!(
                    self.classify_visibility(&method.vis),
                    VisibilityLevel::Pub | VisibilityLevel::PubCrate
                );

                if is_trait_impl {
                    self.counts.local_trait_methods += 1;
                    if !is_pure {
                        self.counts.local_trait_impure += 1;
                    }
                } else {
                    self.counts.inherent_methods += 1;
                    if !is_pure {
                        self.counts.inherent_impure += 1;
                    }
                }

                self.functions.push(FunctionInfo {
                    name: method.sig.ident.to_string(),
                    file: self.current_file.clone(),
                    is_pure,
                    is_public: is_pub,
                    hidden_deps,
                    has_trait_seam,
                });
            }
        }
    }

    fn is_impl_method_impure(&self, method: &syn::ImplItemFn) -> bool {
        if self.has_mut_param(&method.sig) {
            return true;
        }
        if self.is_unit_return(&method.sig) {
            return true;
        }
        if method.sig.unsafety.is_some() {
            return true;
        }
        self.has_unsafe_block(&method.block) || self.has_io_call(&method.block)
    }

    fn is_foreign_trait(&self, path: &syn::Path) -> bool {
        if let Some(last) = path.segments.last() {
            let name = last.ident.to_string();
            if KNOWN_FOREIGN_TRAITS.contains(&name.as_str()) {
                return true;
            }
        }
        if path.segments.len() > 1 {
            if let Some(first) = path.segments.first() {
                let name = first.ident.to_string();
                return name == "std" || name == "core" || name == "alloc";
            }
        }
        false
    }



    fn classify_visibility(&self, vis: &Visibility) -> VisibilityLevel {
        match vis {
            Visibility::Public(_) => VisibilityLevel::Pub,
            Visibility::Restricted(_) => VisibilityLevel::PubCrate,
            _ => VisibilityLevel::Private,
        }
    }

    fn has_test_attr(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let tokens = attr.to_token_stream().to_string();
            let path = attr.path().get_ident().map(|i| i.to_string());
            matches!(path.as_deref(), Some("cfg")) && tokens.contains("test")
                || matches!(path.as_deref(), Some("test"))
                || matches!(path.as_deref(), Some("cfg_attr")) && tokens.contains("test")
        })
    }

    fn is_probably_pure(&self, item_fn: &ItemFn) -> bool {
        if self.has_mut_param(&item_fn.sig) {
            return false;
        }
        if self.is_unit_return(&item_fn.sig) {
            return false;
        }
        if item_fn.sig.unsafety.is_some() {
            return false;
        }
        !self.has_unsafe_block(&item_fn.block)
    }

    fn has_mut_param(&self, sig: &syn::Signature) -> bool {
        sig.inputs.iter().any(|arg| match arg {
            syn::FnArg::Receiver(recv) => recv.mutability.is_some(),
            syn::FnArg::Typed(pat_type) => self.has_mut_in_type(&pat_type.ty),
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn has_mut_in_type(&self, ty: &syn::Type) -> bool {
        use syn::Type;
        match ty {
            Type::Reference(reference) => reference.mutability.is_some(),
            Type::Paren(inner) => self.has_mut_in_type(&inner.elem),
            _ => false,
        }
    }

    fn is_unit_return(&self, sig: &syn::Signature) -> bool {
        match &sig.output {
            syn::ReturnType::Default => true,
            syn::ReturnType::Type(_, ty) => {
                if let syn::Type::Tuple(tuple) = ty.as_ref() {
                    tuple.elems.is_empty()
                } else {
                    false
                }
            }
        }
    }

    fn has_unsafe_block(&self, block: &syn::Block) -> bool {
        let mut finder = UnsafeFinder::new();
        finder.visit_block(block);
        finder.found
    }

    fn has_io_call(&self, block: &syn::Block) -> bool {
        let mut finder = crate::io_call_finder::IoCallFinder::new();
        finder.visit_block(block);
        finder.found
    }

    fn count_hidden_deps_in_block(&self, block: &syn::Block) -> usize {
        let mut finder = crate::hidden_dep_finder::HiddenDepFinder::new();
        finder.visit_block(block);
        finder.count
    }

    fn count_hidden_deps_in_impl_method(&self, method: &syn::ImplItemFn, concrete_fields: Vec<String>) -> usize {
        let mut finder = crate::hidden_dep_finder::HiddenDepFinder::new();
        finder.set_concrete_fields(concrete_fields);
        finder.visit_block(&method.block);
        finder.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibilityLevel {
    Private,
    PubCrate,
    Pub,
}
