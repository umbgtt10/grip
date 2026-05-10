// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use quote::ToTokens;
use syn::visit::Visit;
use syn::{Attribute, Item, ItemFn, Visibility};

use crate::item_counts::ItemCounts;
use crate::unsafe_finder::UnsafeFinder;

#[derive(Debug)]
pub struct Collector {
    counts: ItemCounts,
}

impl Collector {
    fn new() -> Self {
        Self {
            counts: ItemCounts::default(),
        }
    }

    pub fn collect(source: &str) -> ItemCounts {
        let syntax = match syn::parse_file(source) {
            Ok(s) => s,
            Err(_) => return ItemCounts::default(),
        };

        let mut collector = Self::new();
        for item in &syntax.items {
            collector.visit_item(item);
        }
        collector.counts
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
            _ => {}
        }
    }
}

impl Collector {
    fn visit_fn(&mut self, item_fn: &ItemFn) {
        self.counts.total_functions += 1;
        self.counts.total_items += 1;
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
        if self.is_probably_pure(item_fn) {
            self.counts.pure_functions += 1;
        }
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
        sig.inputs.iter().any(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                self.has_mut_in_type(&pat_type.ty)
            } else {
                false
            }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibilityLevel {
    Private,
    PubCrate,
    Pub,
}
