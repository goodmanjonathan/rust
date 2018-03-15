// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module implements some validity checks for attributes.
//! In particular it verifies that `#[inline]` and `#[repr]` attributes are
//! attached to items that actually support them and if there are
//! conflicts between multiple such attributes attached to the same
//! item.

use ty::TyCtxt;

use syntax::ast;
use syntax_pos::Span;
use hir;
use hir::intravisit::{self, Visitor, NestedVisitorMap};

/// An abstraction over anything that an attribute may be affixed to.
#[derive(Debug, Copy, Clone)]
enum AttributeTarget<'a> {
    ExternCrate(&'a hir::Item),
    Use(&'a hir::Item),
    Static(&'a hir::Item),
    Const(&'a hir::Item),
    Fn(&'a hir::Item),
    Mod(&'a hir::Item),
    ForeignMod(&'a hir::Item),
    GlobalAsm(&'a hir::Item),
    TypeAlias(&'a hir::Item),
    Enum(&'a hir::Item),
    Struct(&'a hir::Item),
    Union(&'a hir::Item),
    Trait(&'a hir::Item),
    TraitAlias(&'a hir::Item),
    Impl(&'a hir::Item),

    TraitAssociatedConst(&'a hir::TraitItem),
    TraitMethod(&'a hir::TraitItem),
    TraitAssociatedType(&'a hir::TraitItem),

    ImplAssociatedConst(&'a hir::ImplItem),
    ImplMethod(&'a hir::ImplItem),
    ImplAssociatedTyoe(&'a hir::ImplItem),

    ForeignFn(&'a hir::ForeignItem),
    ForeignStatic(&'a hir::ForeignItem),
    ForeignType(&'a hir::ForeignItem),

    Asm,

    Crate(&'a hir::Crate),

    Variant,

    Field,

    MacroDef(&'a hir::MacroDef),

    MatchArm(&'a hir::Arm),

    Stmt(&'a hir::Stmt),

    Expr(&'a hir::Expr),
}

impl<'a> AttributeTarget<'a> {
    fn id(self) -> hir::NodeId {
        match self {
            AttributeTarget::Item(item) => item.id,
            AttributeTarget::TraitItem(item) => item.id,
            AttributeTarget::ImplItem(item) => item.id,
            AttributeTarget::ForeignItem(item) => item.id,
        }
    }

    fn attrs(self) -> &'a hir::HirVec<ast::Attribute> {
        match self {
            AttributeTarget::Item(item) => &item.attrs,
            AttributeTarget::TraitItem(item) => &item.attrs,
            AttributeTarget::ImplItem(item) => &item.attrs,
            AttributeTarget::ForeignItem(item) => &item.attrs,
        }
    }

    fn span(self) -> Span {
        match self {
            AttributeTarget::Item(item) => item.span,
            AttributeTarget::TraitItem(item) => item.span,
            AttributeTarget::ImplItem(item) => item.span,
            AttributeTarget::ForeignItem(item) => item.span,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Target {
    Fn,
    Struct,
    Union,
    Enum,
    Other,
}

impl Target {
    fn from_item(item: AttributeTarget) -> Target {
        match item {
            AttributeTarget::Item(item) => {
                match item.node {
                    hir::ItemFn(..) => Target::Fn,
                    hir::ItemStruct(..) => Target::Struct,
                    hir::ItemUnion(..) => Target::Union,
                    hir::ItemEnum(..) => Target::Enum,
                    _ => Target::Other,
                }
            }
            AttributeTarget::TraitItem(item) => {
                match item.node {
                    hir::TraitItemKind::Method(..) => Target::Fn,
                    _ => Target::Other,
                }
            }
            AttributeTarget::ImplItem(item) => {
                match item.node {
                    hir::ImplItemKind::Method(..) => Target::Fn,
                    _ => Target::Other,
                }
            }
            AttributeTarget::ForeignItem(item) => {
                match item.node {
                    hir::ForeignItem_::ForeignItemFn(..) => Target::Fn,
                    _ => Target::Other,
                }
            }
        }
    }
}

struct CheckAttrVisitor<'a, 'tcx: 'a> {
    tcx: TyCtxt<'a, 'tcx, 'tcx>,
}

impl<'a, 'tcx> CheckAttrVisitor<'a, 'tcx> {
    /// Check any attribute.
    fn check_attributes(&self, item: AttributeTarget, target: Target) {
        if target == Target::Fn {
            self.tcx.trans_fn_attrs(self.tcx.hir.local_def_id(item.id()));
        } else if let Some(a) = item.attrs().iter().find(|a| a.check_name("target_feature")) {
            self.tcx.sess.struct_span_err(a.span, "attribute should be applied to a function")
                .span_label(item.span(), "not a function")
                .emit();
        }

        for attr in item.attrs() {
            if let Some(name) = attr.name() {
                if name == "inline" {
                    self.check_inline(attr, item, target);
                } else if name == "non_exhaustive" {
                    self.check_non_exhaustive(attr, item, target);
                }
            }
        }

        self.check_repr(item, target);
    }

    /// Check if an `#[inline]` is applied to a function.
    fn check_inline(&self, attr: &hir::Attribute, item: AttributeTarget, target: Target) {
        if target != Target::Fn {
            struct_span_err!(self.tcx.sess,
                             attr.span,
                             E0518,
                             "attribute should be applied to function")
                .span_label(item.span(), "not a function")
                .emit();
        }
    }

    fn check_non_exhaustive(&self, attr: &hir::Attribute, item: AttributeTarget, target: Target) {
        if target != Target::Struct && target != Target::Enum {
            struct_span_err!(self.tcx.sess,
                             attr.span,
                             E0698,
                             "attribute should be applied to struct or enum definition")
                .span_label(item.span(), "not a struct or enum definition")
                .emit();
        }
    }

    /// Check if the `#[repr]` attributes on `item` are valid.
    fn check_repr(&self, item: AttributeTarget, target: Target) {
        // Extract the names of all repr hints, e.g., [foo, bar, align] for:
        // ```
        // #[repr(foo)]
        // #[repr(bar, align(8))]
        // ```
        let hints: Vec<_> = item.attrs()
            .iter()
            .filter(|attr| match attr.name() {
                Some(name) => name == "repr",
                None => false,
            })
            .filter_map(|attr| attr.meta_item_list())
            .flat_map(|hints| hints)
            .collect();

        let mut int_reprs = 0;
        let mut is_c = false;
        let mut is_simd = false;
        let mut is_transparent = false;

        for hint in &hints {
            let name = if let Some(name) = hint.name() {
                name
            } else {
                // Invalid repr hint like repr(42). We don't check for unrecognized hints here
                // (libsyntax does that), so just ignore it.
                continue;
            };

            let (article, allowed_targets) = match &*name.as_str() {
                "C" => {
                    is_c = true;
                    if target != Target::Struct &&
                            target != Target::Union &&
                            target != Target::Enum {
                                ("a", "struct, enum or union")
                    } else {
                        continue
                    }
                }
                "packed" => {
                    if target != Target::Struct &&
                            target != Target::Union {
                                ("a", "struct or union")
                    } else {
                        continue
                    }
                }
                "simd" => {
                    is_simd = true;
                    if target != Target::Struct {
                        ("a", "struct")
                    } else {
                        continue
                    }
                }
                "align" => {
                    if target != Target::Struct &&
                            target != Target::Union {
                        ("a", "struct or union")
                    } else {
                        continue
                    }
                }
                "transparent" => {
                    is_transparent = true;
                    if target != Target::Struct {
                        ("a", "struct")
                    } else {
                        continue
                    }
                }
                "i8" | "u8" | "i16" | "u16" |
                "i32" | "u32" | "i64" | "u64" |
                "isize" | "usize" => {
                    int_reprs += 1;
                    if target != Target::Enum {
                        ("an", "enum")
                    } else {
                        continue
                    }
                }
                _ => continue,
            };
            struct_span_err!(self.tcx.sess, hint.span, E0517,
                             "attribute should be applied to {}", allowed_targets)
                .span_label(item.span(), format!("not {} {}", article, allowed_targets))
                .emit();
        }

        // Just point at all repr hints if there are any incompatibilities.
        // This is not ideal, but tracking precisely which ones are at fault is a huge hassle.
        let hint_spans = hints.iter().map(|hint| hint.span);

        // Error on repr(transparent, <anything else>).
        if is_transparent && hints.len() > 1 {
            let hint_spans: Vec<_> = hint_spans.clone().collect();
            span_err!(self.tcx.sess, hint_spans, E0692,
                      "transparent struct cannot have other repr hints");
        }
        // Warn on repr(u8, u16), repr(C, simd), and c-like-enum-repr(C, u8)
        if (int_reprs > 1)
           || (is_simd && is_c)
           || (int_reprs == 1 && is_c && is_c_like_enum(item)) {
            let hint_spans: Vec<_> = hint_spans.collect();
            span_warn!(self.tcx.sess, hint_spans, E0566,
                       "conflicting representation hints");
        }
    }
}

impl<'a, 'tcx> Visitor<'tcx> for CheckAttrVisitor<'a, 'tcx> {
    fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
        NestedVisitorMap::None
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        let item_like = AttributeTarget::Item(item);
        let target = Target::from_item(item_like);
        self.check_attributes(item_like, target);
        intravisit::walk_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'tcx hir::TraitItem) {
        let item_like = AttributeTarget::TraitItem(item);
        let target = Target::from_item(item_like);
        self.check_attributes(item_like, target);
        intravisit::walk_trait_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'tcx hir::ImplItem) {
        let item_like = AttributeTarget::ImplItem(item);
        let target = Target::from_item(item_like);
        self.check_attributes(item_like, target);
        intravisit::walk_impl_item(self, item);
    }

    fn visit_foreign_item(&mut self, item: &'tcx hir::ForeignItem) {
        let item_like = AttributeTarget::ForeignItem(item);
        let target = Target::from_item(item_like);
        self.check_attributes(item_like, target);
        intravisit::walk_foreign_item(self, item);
    }

    fn visit_generic_param(&mut self, param: &hir::GenericParam) {

    }

    fn visit_macro_def(&mut self, macro_def: &hir::MacroDef) {
    }
}

pub fn check_crate<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>) {
    let mut checker = CheckAttrVisitor { tcx };
    tcx.hir.krate().visit_all_item_likes(&mut checker.as_deep_visitor());
}

fn is_c_like_enum(item: AttributeTarget) -> bool {
    if let AttributeTarget::Item(item) = item {
        if let hir::ItemEnum(ref def, _) = item.node {
            for variant in &def.variants {
                match variant.node.data {
                    hir::VariantData::Unit(_) => { /* continue */ }
                    _ => { return false; }
                }
            }
            return true;
        }
    }
    false
}
