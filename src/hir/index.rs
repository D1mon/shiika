/// Index of all the classes and methods
///
/// Note: `MethodSignature` contained in `Index` is "as is" and
/// may be wrong (eg. its return type does not exist).
/// It is checked in `HirMaker`.
use std::collections::HashMap;
use crate::ast;
use crate::error;
use crate::error::*;
use crate::hir::*;
use crate::ty::*;
use crate::names::*;

#[derive(Debug, PartialEq)]
pub struct Index {
    pub body: IndexBody
}
type IndexBody = HashMap<ClassFullname, HashMap<MethodName, MethodSignature>>;

impl Index {
    pub fn new(stdlib: &Vec<SkClass>, toplevel_defs: &Vec<ast::Definition>) -> Result<Index, Error> {
        let mut body = HashMap::new();

        index_stdlib(&mut body, stdlib);
        index_program(&mut body, toplevel_defs)?;

        Ok(Index { body })
    }

    pub fn get(&self, class_fullname: &ClassFullname) -> Option<&HashMap<MethodName, MethodSignature>> {
        self.body.get(class_fullname)
    }

    pub fn find_method(&self, class_fullname: &ClassFullname, method_name: &MethodName) -> Option<&MethodSignature> {
        self.body.get(class_fullname).and_then(|methods| methods.get(method_name))
    }
}

fn index_stdlib(body: &mut IndexBody, stdlib: &Vec<SkClass>) {
    stdlib.iter().for_each(|sk_class| {
        let mut sk_methods = HashMap::new();
        sk_class.methods.iter().for_each(|sk_method| {
            sk_methods.insert(sk_method.signature.name.clone(),
                              sk_method.signature.clone());
        });
        body.insert(sk_class.fullname.clone(), sk_methods);
    });
}

fn index_program(body: &mut IndexBody, toplevel_defs: &Vec<ast::Definition>) -> Result<(), Error> {
    toplevel_defs.iter().try_for_each(|def| {
        match def {
            ast::Definition::ClassDefinition { name, defs } => {
                index_class(body, &name, &defs);
                Ok(())
            },
            _ => {
                Err(error::syntax_error(&format!("must not be toplevel: {:?}", def)))
            }
        }
    })
}

fn index_class(body: &mut IndexBody, name: &ClassName, defs: &Vec<ast::Definition>) {
    let class_fullname = name.to_class_fullname(); // TODO: nested class
    let instance_ty = ty::raw(&class_fullname.0);
    let class_ty = instance_ty.meta_ty();

    let metaclass_fullname = class_ty.fullname;
    let mut instance_methods = HashMap::new();
    let mut class_methods = HashMap::new();

    defs.iter().for_each(|def| {
        match def {
            ast::Definition::InstanceMethodDefinition { sig, .. } => {
                let hir_sig = crate::hir::create_signature(class_fullname.to_string(), sig);
                instance_methods.insert(sig.name.clone(), hir_sig);
            },
            ast::Definition::ClassMethodDefinition { sig, .. } => {
                let hir_sig = crate::hir::create_signature(metaclass_fullname.to_string(), sig);
                class_methods.insert(sig.name.clone(), hir_sig);
            },
            _ => panic!("TODO")
        }
    });

    body.insert(class_fullname, instance_methods);
    body.insert(metaclass_fullname, class_methods);
}