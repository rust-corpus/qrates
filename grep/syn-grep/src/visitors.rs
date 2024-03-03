#[derive(Debug, Default)]
pub(super) struct UnsafeBlockReport {
    pub(super) expression_count: u64,
}

#[derive(Debug, Default)]
pub(super) struct FunctionReport {
    pub(super) function_name: String,
    pub(super) is_unsafe: bool,
    pub(super) expression_count: u64,
    pub(super) unsafe_blocks: Vec<UnsafeBlockReport>,
}

#[derive(Debug, Default)]
pub(super) struct FunctionVisitor {
    expression_count: u64,
    pub(super) unsafe_blocks: Vec<UnsafeBlockReport>,
    pub(super) functions: Vec<FunctionReport>,
}

impl FunctionVisitor {
    fn after_visit_fn(&mut self, function_name: String, is_unsafe: bool, expression_count: u64) {
        if !self.unsafe_blocks.is_empty() {
            let unsafe_blocks = std::mem::take(&mut self.unsafe_blocks);
            let function = FunctionReport {
                function_name: function_name,
                is_unsafe,
                expression_count,
                unsafe_blocks,
            };
            self.functions.push(function);
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, item: &syn::ItemFn) {
        let old_unsafe_blocks = std::mem::take(&mut self.unsafe_blocks);
        let old_expression_count = self.expression_count;
        syn::visit::visit_item_fn(self, item);
        self.after_visit_fn(
            item.sig.ident.to_string(),
            item.sig.unsafety.is_some(),
            self.expression_count - old_expression_count,
        );
        self.unsafe_blocks = old_unsafe_blocks;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let old_unsafe_blocks = std::mem::take(&mut self.unsafe_blocks);
        let old_expression_count = self.expression_count;
        syn::visit::visit_impl_item_fn(self, item);
        self.after_visit_fn(
            item.sig.ident.to_string(),
            item.sig.unsafety.is_some(),
            self.expression_count - old_expression_count,
        );
        self.unsafe_blocks = old_unsafe_blocks;
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        let old_unsafe_blocks = std::mem::take(&mut self.unsafe_blocks);
        let old_expression_count = self.expression_count;
        syn::visit::visit_trait_item_fn(self, item);
        self.after_visit_fn(
            item.sig.ident.to_string(),
            item.sig.unsafety.is_some(),
            self.expression_count - old_expression_count,
        );
        self.unsafe_blocks = old_unsafe_blocks;
    }

    fn visit_expr_unsafe(&mut self, block: &'ast syn::ExprUnsafe) {
        let old_expression_count = self.expression_count;
        syn::visit::visit_expr_unsafe(self, block);
        let size = self.expression_count - old_expression_count;
        let unsafe_block_report = UnsafeBlockReport {
            expression_count: size,
        };
        self.unsafe_blocks.push(unsafe_block_report);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        self.expression_count = self.expression_count.checked_add(1).unwrap();
        syn::visit::visit_expr(self, expr);
    }
}
