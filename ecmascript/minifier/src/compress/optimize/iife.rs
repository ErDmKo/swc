use super::Optimizer;
use crate::{
    compress::optimize::Ctx,
    mode::Mode,
    util::{idents_used_by, make_number},
};
use fxhash::FxHashMap;
use std::{
    collections::HashMap,
    mem::{replace, swap},
};
use swc_atoms::js_word;
use swc_common::{pass::Either, Spanned, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_transforms_base::ext::MapWithMut;
use swc_ecma_utils::{ident::IdentLike, undefined, ExprFactory, Id};
use swc_ecma_visit::VisitMutWith;

/// Methods related to the option `negate_iife`.
impl<M> Optimizer<'_, M>
where
    M: Mode,
{
    /// Negates iife, while ignore return value.
    pub(super) fn negate_iife_ignoring_ret(&mut self, e: &mut Expr) {
        if !self.options.negate_iife || self.ctx.in_bang_arg || self.ctx.dont_use_negated_iife {
            return;
        }

        let expr = match e {
            Expr::Call(e) => e,
            _ => return,
        };

        let callee = match &mut expr.callee {
            ExprOrSuper::Super(_) => return,
            ExprOrSuper::Expr(e) => &mut **e,
        };

        match callee {
            Expr::Fn(..) => {
                log::debug!("negate_iife: Negating iife");
                *e = Expr::Unary(UnaryExpr {
                    span: DUMMY_SP,
                    op: op!("!"),
                    arg: Box::new(e.take()),
                });
                return;
            }
            _ => {}
        }
    }

    /// Returns true if it did any work.
    ///
    ///
    /// - `iife ? foo : bar` => `!iife ? bar : foo`
    pub(super) fn negate_iife_in_cond(&mut self, e: &mut Expr) -> bool {
        let cond = match e {
            Expr::Cond(v) => v,
            _ => return false,
        };

        let test_call = match &mut *cond.test {
            Expr::Call(e) => e,
            _ => return false,
        };

        let callee = match &mut test_call.callee {
            ExprOrSuper::Super(_) => return false,
            ExprOrSuper::Expr(e) => &mut **e,
        };

        match callee {
            Expr::Fn(..) => {
                log::debug!("negate_iife: Swapping cons and alt");
                cond.test = Box::new(Expr::Unary(UnaryExpr {
                    span: DUMMY_SP,
                    op: op!("!"),
                    arg: cond.test.take(),
                }));
                swap(&mut cond.cons, &mut cond.alt);
                return true;
            }
            _ => false,
        }
    }

    pub(super) fn restore_negated_iife(&mut self, cond: &mut CondExpr) {
        if !self.ctx.dont_use_negated_iife {
            return;
        }

        match &mut *cond.test {
            Expr::Unary(UnaryExpr {
                op: op!("!"), arg, ..
            }) => match &mut **arg {
                Expr::Call(CallExpr {
                    span: call_span,
                    callee: ExprOrSuper::Expr(callee),
                    args,
                    ..
                }) => match &**callee {
                    Expr::Fn(..) => {
                        cond.test = Box::new(Expr::Call(CallExpr {
                            span: *call_span,
                            callee: callee.take().as_callee(),
                            args: args.take(),
                            type_args: Default::default(),
                        }));
                        swap(&mut cond.cons, &mut cond.alt);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        };
    }
}

/// Methods related to iife.
impl<M> Optimizer<'_, M>
where
    M: Mode,
{
    /// # Exmaple
    ///
    /// ## Input
    ///
    /// ```ts
    /// (function(x) {
    ///     (function(y) {
    ///         console.log(7);
    ///     })(7);
    /// })(7);
    /// ```
    ///
    ///
    /// ## Output
    ///
    /// ```ts
    /// (function(x) {
    ///     (function(y) {
    ///         console.log(y);
    ///     })(x);
    /// })(7);
    /// ```
    pub(super) fn inline_args_of_iife(&mut self, e: &mut CallExpr) {
        if self.options.inline == 0 {
            return;
        }

        let has_spread_arg = e.args.iter().any(|v| v.spread.is_some());
        if has_spread_arg {
            return;
        }

        let callee = match &mut e.callee {
            ExprOrSuper::Super(_) => return,
            ExprOrSuper::Expr(e) => &mut **e,
        };

        fn find_params(callee: &Expr) -> Option<Vec<&Pat>> {
            match callee {
                Expr::Arrow(callee) => Some(callee.params.iter().collect()),
                Expr::Fn(callee) => Some(
                    callee
                        .function
                        .params
                        .iter()
                        .map(|param| &param.pat)
                        .collect(),
                ),
                _ => return None,
            }
        }

        fn find_body(callee: &mut Expr) -> Option<Either<&mut BlockStmt, &mut Expr>> {
            match callee {
                Expr::Arrow(e) => match &mut e.body {
                    BlockStmtOrExpr::BlockStmt(b) => Some(Either::Left(b)),
                    BlockStmtOrExpr::Expr(b) => Some(Either::Right(&mut **b)),
                },
                Expr::Fn(e) => Some(Either::Left(e.function.body.as_mut().unwrap())),
                _ => None,
            }
        }

        let params = find_params(&callee);
        if let Some(params) = params {
            let mut vars = HashMap::default();
            // We check for parameter and argument
            for (idx, param) in params.iter().enumerate() {
                let arg = e.args.get(idx).map(|v| &v.expr);
                if let Pat::Ident(param) = &param {
                    if let Some(usage) = self
                        .data
                        .as_ref()
                        .and_then(|data| data.vars.get(&param.to_id()))
                    {
                        if usage.reassigned {
                            continue;
                        }
                        if usage.ref_count != 1 {
                            continue;
                        }
                    }

                    if let Some(arg) = arg {
                        // NOTE
                        //
                        // This function is misdesigned and should be removed.
                        // This is wrong because the order of execution is not guaranteed.
                        match &**arg {
                            Expr::Ident(..) | Expr::Lit(..) => {}
                            _ => continue,
                        }

                        let should_be_inlined = self.can_be_inlined_for_iife(arg);
                        if should_be_inlined {
                            log::debug!(
                                "iife: Trying to inline argument ({}{:?})",
                                param.id.sym,
                                param.id.span.ctxt
                            );
                            vars.insert(param.to_id(), arg.clone());
                        }
                    } else {
                        log::debug!(
                            "iife: Trying to inline argument ({}{:?}) (undefined)",
                            param.id.sym,
                            param.id.span.ctxt
                        );

                        vars.insert(param.to_id(), undefined(param.span()));
                    }
                }
            }

            match find_body(callee) {
                Some(Either::Left(body)) => {
                    log::debug!("inline: Inlining arguments");
                    self.inline_vars_in_node(body, vars);
                }
                Some(Either::Right(body)) => {
                    log::debug!("inline: Inlining arguments");
                    self.inline_vars_in_node(body, vars);
                }
                _ => {}
            }
        }
    }

    pub(super) fn inline_vars_in_node<N>(&mut self, n: &mut N, vars: FxHashMap<Id, Box<Expr>>)
    where
        N: VisitMutWith<Self>,
    {
        log::debug!("inline: inline_vars_in_node");
        let ctx = Ctx {
            inline_prevented: false,
            ..self.ctx
        };
        let orig_vars = replace(&mut self.state.vars_for_inlining, vars);
        n.visit_mut_with(&mut *self.with_ctx(ctx));
        self.state.vars_for_inlining = orig_vars;
    }

    /// Fully inlines iife.
    ///
    /// # Example
    ///
    /// ## Input
    ///
    /// ```ts
    /// (function () {
    ///     return {};
    /// })().x = 10;
    /// ```
    ///
    /// ## Oupuy
    ///
    /// ```ts
    /// ({
    /// }).x = 10;
    /// ```
    pub(super) fn invoke_iife(&mut self, e: &mut Expr) {
        if self.options.inline == 0 {
            let skip = match e {
                Expr::Call(v) => !v.callee.span().is_dummy(),
                _ => true,
            };

            if skip {
                return;
            }
        }

        let call = match e {
            Expr::Call(v) => v,
            _ => return,
        };

        if self.has_noinline(call.span) {
            return;
        }

        let callee = match &mut call.callee {
            ExprOrSuper::Super(_) => return,
            ExprOrSuper::Expr(e) => &mut **e,
        };

        if self.ctx.inline_prevented {
            log::trace!("iife: [x] Inline is prevented");
            return;
        }

        match callee {
            Expr::Arrow(f) => {
                if f.is_async {
                    log::trace!("iife: [x] Cannot inline async fn");
                    return;
                }

                if f.is_generator {
                    log::trace!("iife: [x] Cannot inline generator");
                    return;
                }

                if self.ctx.in_top_level() && !self.ctx.in_call_arg && self.options.negate_iife {
                    match &f.body {
                        BlockStmtOrExpr::BlockStmt(body) => {
                            let has_decl = body.stmts.iter().any(|stmt| match stmt {
                                Stmt::Decl(..) => true,
                                _ => false,
                            });
                            if has_decl {
                                return;
                            }
                        }
                        BlockStmtOrExpr::Expr(_) => {}
                    }
                }

                if f.params.iter().any(|param| !param.is_ident()) {
                    return;
                }

                let param_ids = f
                    .params
                    .iter()
                    .map(|p| p.clone().ident().unwrap().id)
                    .collect::<Vec<_>>();

                match &mut f.body {
                    BlockStmtOrExpr::BlockStmt(body) => {
                        let new = self.inline_fn_like(&param_ids, body, &mut call.args);
                        if let Some(new) = new {
                            self.changed = true;
                            log::debug!("inline: Inlining a function call (arrow)");

                            *e = new;
                        }
                        return;
                    }
                    BlockStmtOrExpr::Expr(body) => match &**body {
                        Expr::Lit(Lit::Num(..)) => {
                            if self.ctx.in_obj_of_non_computed_member {
                                return;
                            }
                        }
                        _ => {}
                    },
                }

                match &mut f.body {
                    BlockStmtOrExpr::BlockStmt(_) => {
                        // TODO
                    }
                    BlockStmtOrExpr::Expr(body) => {
                        self.changed = true;

                        {
                            let vars = f
                                .params
                                .iter()
                                .cloned()
                                .map(|name| VarDeclarator {
                                    span: DUMMY_SP.apply_mark(self.marks.non_top_level),
                                    name,
                                    init: Default::default(),
                                    definite: Default::default(),
                                })
                                .collect::<Vec<_>>();

                            if !vars.is_empty() {
                                self.prepend_stmts.push(Stmt::Decl(Decl::Var(VarDecl {
                                    span: DUMMY_SP,
                                    kind: VarDeclKind::Var,
                                    declare: Default::default(),
                                    decls: vars,
                                })));
                            }
                        }

                        let mut exprs = vec![];
                        exprs.push(Box::new(make_number(DUMMY_SP, 0.0)));
                        for (idx, param) in f.params.iter().enumerate() {
                            if let Some(arg) = call.args.get_mut(idx) {
                                exprs.push(Box::new(Expr::Assign(AssignExpr {
                                    span: DUMMY_SP.apply_mark(self.marks.non_top_level),
                                    op: op!("="),
                                    left: PatOrExpr::Pat(Box::new(param.clone())),
                                    right: arg.expr.take(),
                                })));
                            }
                        }

                        if call.args.len() > f.params.len() {
                            for arg in &mut call.args[f.params.len()..] {
                                exprs.push(arg.expr.take());
                            }
                        }
                        exprs.push(body.take());

                        log::debug!("inline: Inlining a call to an arrow function");
                        *e = Expr::Seq(SeqExpr {
                            span: DUMMY_SP,
                            exprs,
                        });
                        return;
                    }
                }
            }
            Expr::Fn(f) => {
                if self.ctx.in_top_level() && !self.ctx.in_call_arg && self.options.negate_iife {
                    let body = f.function.body.as_ref().unwrap();
                    let has_decl = body.stmts.iter().any(|stmt| match stmt {
                        Stmt::Decl(..) => true,
                        _ => false,
                    });
                    if has_decl {
                        log::trace!("iife: [x] Found decl");
                        return;
                    }
                }

                if f.function.is_async {
                    log::trace!("iife: [x] Cannot inline async fn");
                    return;
                }

                if f.function.is_generator {
                    log::trace!("iife: [x] Cannot inline generator");
                    return;
                }

                // Abort if a parameter is complex
                if f.function.params.iter().any(|param| match param.pat {
                    Pat::Object(..) | Pat::Array(..) | Pat::Assign(..) | Pat::Rest(..) => true,
                    _ => false,
                }) {
                    log::trace!("iife: [x] Found complex pattern");
                    return;
                }

                if let Some(i) = &f.ident {
                    if idents_used_by(&f.function.body).contains(&i.to_id()) {
                        log::trace!("iife: [x] Recursive?");
                        return;
                    }
                }

                for arg in &call.args {
                    if arg.spread.is_some() {
                        log::trace!("iife: [x] Found spread argument");
                        return;
                    }
                    match &*arg.expr {
                        Expr::Fn(..) | Expr::Arrow(..) => {
                            log::trace!("iife: [x] Found callable argument");
                            return;
                        }
                        _ => {}
                    }
                }

                let body = f.function.body.as_mut().unwrap();
                if body.stmts.is_empty() {
                    *e = *undefined(f.function.span);
                    return;
                }

                let param_ids = f
                    .function
                    .params
                    .iter()
                    .map(|p| p.pat.clone().ident().unwrap().id)
                    .collect::<Vec<_>>();

                if !self.can_inline_fn_like(&param_ids, body) {
                    log::trace!("iife: [x] Body is not inliable");
                    return;
                }

                let new = self.inline_fn_like(&param_ids, body, &mut call.args);
                if let Some(new) = new {
                    self.changed = true;
                    log::debug!("inline: Inlining a function call");

                    *e = new;
                }

                //
            }
            _ => {}
        }
    }

    fn can_inline_fn_like(&self, param_ids: &[Ident], body: &BlockStmt) -> bool {
        if !body.stmts.iter().all(|stmt| match stmt {
            Stmt::Decl(Decl::Var(VarDecl {
                kind: VarDeclKind::Var | VarDeclKind::Let,
                decls,
                ..
            })) => {
                if decls.iter().any(|decl| match decl.name {
                    Pat::Ident(..) => false,
                    _ => true,
                }) {
                    return false;
                }

                if self.ctx.executed_multiple_time {
                    return false;
                }

                true
            }

            Stmt::Expr(e) => match &*e.expr {
                Expr::Await(..) => false,

                // TODO: Check if paramter is used and inline if call is not related to parameters.
                Expr::Call(e) => {
                    let used = idents_used_by(e);
                    param_ids.iter().all(|param| !used.contains(&param.to_id()))
                }

                _ => true,
            },

            Stmt::Return(ReturnStmt { arg, .. }) => match arg.as_deref() {
                Some(Expr::Await(..)) => false,

                Some(Expr::Lit(Lit::Num(..))) => {
                    if self.ctx.in_obj_of_non_computed_member {
                        false
                    } else {
                        true
                    }
                }
                _ => true,
            },
            _ => false,
        }) {
            return false;
        }

        if idents_used_by(&*body)
            .iter()
            .any(|v| v.0 == js_word!("arguments"))
        {
            return false;
        }

        true
    }

    fn inline_fn_like(
        &mut self,
        params: &[Ident],
        body: &mut BlockStmt,
        args: &mut [ExprOrSpread],
    ) -> Option<Expr> {
        if !self.can_inline_fn_like(&params, &*body) {
            return None;
        }

        self.changed = true;
        log::debug!("inline: Inling an iife");

        let mut exprs = vec![];

        {
            let vars = params
                .iter()
                .cloned()
                .map(BindingIdent::from)
                .map(Pat::Ident)
                .map(|name| VarDeclarator {
                    span: DUMMY_SP.apply_mark(self.marks.non_top_level),
                    name,
                    init: Default::default(),
                    definite: Default::default(),
                })
                .collect::<Vec<_>>();

            if !vars.is_empty() {
                self.prepend_stmts.push(Stmt::Decl(Decl::Var(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Var,
                    declare: Default::default(),
                    decls: vars,
                })));
            }
        }

        for (idx, param) in params.iter().enumerate() {
            if let Some(arg) = args.get_mut(idx) {
                exprs.push(Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP.apply_mark(self.marks.non_top_level),
                    op: op!("="),
                    left: PatOrExpr::Pat(Box::new(Pat::Ident(param.clone().into()))),
                    right: arg.expr.take(),
                })));
            }
        }

        if args.len() > params.len() {
            for arg in &mut args[params.len()..] {
                exprs.push(arg.expr.take());
            }
        }

        for mut stmt in body.stmts.take() {
            match stmt {
                Stmt::Decl(Decl::Var(ref mut var)) => {
                    for decl in &mut var.decls {
                        if decl.init.is_some() {
                            exprs.push(Box::new(Expr::Assign(AssignExpr {
                                span: DUMMY_SP.apply_mark(self.marks.non_top_level),
                                op: op!("="),
                                left: PatOrExpr::Pat(Box::new(decl.name.clone())),
                                right: decl.init.take().unwrap(),
                            })))
                        }
                    }

                    self.prepend_stmts.push(stmt);
                }

                Stmt::Expr(stmt) => {
                    exprs.push(stmt.expr);
                }

                Stmt::Return(stmt) => {
                    let span = stmt.span;
                    let val = *stmt.arg.unwrap_or_else(|| undefined(span));
                    exprs.push(Box::new(val));

                    return Some(Expr::Seq(SeqExpr {
                        span: DUMMY_SP,
                        exprs,
                    }));
                }
                _ => {}
            }
        }

        if let Some(last) = exprs.last_mut() {
            *last = Box::new(Expr::Unary(UnaryExpr {
                span: DUMMY_SP,
                op: op!("void"),
                arg: last.take(),
            }));
        } else {
            return Some(*undefined(body.span));
        }

        Some(Expr::Seq(SeqExpr {
            span: DUMMY_SP,
            exprs,
        }))
    }

    fn can_be_inlined_for_iife(&self, arg: &Expr) -> bool {
        match arg {
            Expr::Lit(..) => true,

            Expr::Unary(UnaryExpr {
                op: op!("void"),
                arg,
                ..
            })
            | Expr::Unary(UnaryExpr {
                op: op!("!"), arg, ..
            }) => self.can_be_inlined_for_iife(&arg),

            Expr::Ident(..) => true,

            Expr::Member(MemberExpr {
                obj: ExprOrSuper::Expr(obj),
                computed: false,
                ..
            }) => self.can_be_inlined_for_iife(&obj),

            Expr::Bin(BinExpr {
                op, left, right, ..
            }) => match op {
                op!(bin, "+") | op!("*") => {
                    self.can_be_inlined_for_iife(&left) && self.can_be_inlined_for_iife(&right)
                }
                _ => false,
            },

            Expr::Object(ObjectLit { props, .. }) => {
                for prop in props {
                    match prop {
                        PropOrSpread::Spread(_) => return false,
                        PropOrSpread::Prop(p) => match &**p {
                            Prop::Shorthand(_) => {}
                            Prop::KeyValue(kv) => {
                                if let PropName::Computed(key) = &kv.key {
                                    if !self.can_be_inlined_for_iife(&key.expr) {
                                        return false;
                                    }
                                }

                                if !self.can_be_inlined_for_iife(&kv.value) {
                                    return false;
                                }
                            }
                            Prop::Assign(p) => {
                                if !self.can_be_inlined_for_iife(&p.value) {
                                    return false;
                                }
                            }
                            _ => return false,
                        },
                    }
                }

                true
            }

            Expr::Arrow(ArrowExpr {
                params,
                body: BlockStmtOrExpr::Expr(body),
                is_async: false,
                is_generator: false,
                ..
            }) => params.iter().all(|p| p.is_ident()) && self.can_be_inlined_for_iife(&body),

            _ => false,
        }
    }
}
