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

    pub fn try_to_load(
        rhai_engine: &mut Engine,
        slug: String,
        source_file: String,
        code: String,
    ) -> Result<Self, String> {
        let mut scope = Scope::new();

        let mut ast = match rhai_engine.compile(&code) {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("rhai \"{}\" syntax error: {}", source_file, e).into());
            }
        };
        ast.set_source(ImmutableString::from(&slug));
        match rhai_engine.run_ast_with_scope(&mut scope, &ast) {
            Ok(()) => (),
            Err(e) => {
                return Err(format!("rhai \"{}\" syntax error: {}", source_file, e).into());
            }
        };

        let shared_controller = ScriptInstanceScope::new(slug.clone());
        let script_instance = ScriptInstance {
            ast: ast,
            scope: scope,
            scope_instance: Rc::new(RefCell::new(shared_controller)),
        };
        script_instance
            .scope
            .push_constant("Main", script_instance.scope_instance.clone());

        Ok(script_instance)
    }

    pub fn run_fn(
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
