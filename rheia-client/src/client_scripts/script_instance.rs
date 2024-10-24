use super::instance_scope::ScriptInstanceScope;
use rhai::{CallFnOptions, Dynamic, Engine, ImmutableString, Scope, AST};
use std::cell::RefCell;
use std::rc::Rc;

pub type RcScopeInstance = Rc<RefCell<ScriptInstanceScope>>;

pub struct ScriptInstance {
    ast: AST,
    scope: Scope<'static>,
    scope_instance: RcScopeInstance,
}

impl ScriptInstance {
    pub fn get_scope_instance(&self) -> &RcScopeInstance {
        &self.scope_instance
    }

    pub fn try_to_load(rhai_engine: &mut Engine, slug: String, code: String) -> Result<Self, String> {
        let mut scope = Scope::new();
        let shared_controller = ScriptInstanceScope::new(slug.clone());
        let scope_instance = Rc::new(RefCell::new(shared_controller));
        scope.push_constant("Main", scope_instance.clone());

        let mut ast = match rhai_engine.compile(&code) {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("rhai \"{}\" syntax error: {}", slug, e).into());
            }
        };
        ast.set_source(ImmutableString::from(slug.clone()));
        match rhai_engine.run_ast_with_scope(&mut scope, &ast) {
            Ok(()) => (),
            Err(e) => {
                return Err(format!("rhai \"{}\" syntax error: {}", slug, e).into());
            }
        };

        Ok(ScriptInstance {
            ast: ast,
            scope: scope,
            scope_instance: scope_instance,
        })
    }

    pub fn _run_fn(
        &mut self,
        rhai_engine: &Engine,
        fn_name: &String,
        attrs: &Vec<Dynamic>,
        bind: &mut Dynamic,
    ) -> Dynamic {
        let options = CallFnOptions::new()
            .eval_ast(false)
            .rewind_scope(true)
            .bind_this_ptr(bind);

        let callback_result =
            rhai_engine.call_fn_with_options::<Dynamic>(options, &mut self.scope, &self.ast, &fn_name, attrs.clone());

        let result = match callback_result {
            Ok(r) => r,
            Err(e) => {
                self.scope_instance
                    .borrow()
                    .console_send(format!("Function {} error: {:?}", fn_name, e));
                Dynamic::from(())
            }
        };
        result
    }
}
