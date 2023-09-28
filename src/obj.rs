use crate::{
    node::NodeWrap,
    parse::{GLOBALS, LOCALS},
    scope::ScopeWrap,
    ty::TyWrap,
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Obj {
    pub next: ObjWrap,
    pub name: &'static str,
    pub offset: i64,
    pub ty: TyWrap,

    pub is_local: bool,
    pub is_function: bool,
    pub is_definition: bool,

    pub body: NodeWrap,
    pub locals: ObjWrap,
    pub stack_size: usize,
    pub params: ObjWrap,
    pub init_data: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ObjWrap {
    pub ptr: Option<*mut Obj>,
}

#[allow(dead_code)]
impl ObjWrap {
    pub fn new(name: &'static str, ty: TyWrap) -> Self {
        let var = Obj {
            next: ObjWrap::empty(),
            name: name,
            offset: 0,
            ty: ty,
            is_local: true,
            is_function: false,
            is_definition: false,
            body: NodeWrap::empty(),
            locals: ObjWrap::empty(),
            stack_size: 0,
            params: ObjWrap::empty(),
            init_data: vec![],
        };
        let var: Option<*mut Obj> = Some(Box::leak(Box::new(var)));
        let var = Self { ptr: var };

        let scope = ScopeWrap::push(name);
        scope.set_var(var);
        return var
    }

    pub fn new_local(name: &'static str, ty: TyWrap) -> Self {
        let var = Self::new(name, ty);
        var.set_nxt(unsafe { LOCALS });
        unsafe { LOCALS = var };
        var
    }

    pub fn new_global(name: &'static str, ty: TyWrap) -> Self {
        let var = Self::new(name, ty);
        var.set_is_local(false);
        var.set_nxt(unsafe { GLOBALS });
        unsafe { GLOBALS = var };
        // ScopeWrap::push(name, var);
        var
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn nxt(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn body(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().body }
    }

    pub fn offset(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().offset }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn locals(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().locals }
    }

    pub fn stack_size(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().stack_size }
    }

    pub fn params(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().params }
    }

    pub fn init_data(&self) -> Vec<usize> {
        let mut v = vec![];
        for c in unsafe { &self.ptr.unwrap().as_ref().unwrap().init_data } {
            v.push(*c);
        }
        v
    }

    pub fn is_local(&self) -> bool {
        unsafe { self.ptr.unwrap().as_ref().unwrap().is_local }
    }

    pub fn is_function(&self) -> bool {
        unsafe { self.ptr.unwrap().as_ref().unwrap().is_function }
    }

    pub fn is_definition(&self) -> bool {
        unsafe { self.ptr.unwrap().as_ref().unwrap().is_definition }
    }


    pub fn set_nxt(&self, next: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_offset(&self, offset: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().offset = offset }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }

    pub fn set_body(&self, body: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().body = body }
    }

    pub fn set_locals(&self, locals: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().locals = locals }
    }

    pub fn set_stack_size(&self, stack_size: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().stack_size = stack_size }
    }

    pub fn set_params(&self, params: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().params = params }
    }

    pub fn set_is_local(&self, is_local: bool) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().is_local = is_local }
    }

    pub fn set_is_function(&self, is_function: bool) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().is_function = is_function }
    }

    pub fn set_is_definition(&self, is_definition: bool) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().is_definition = is_definition }
    }

    pub fn set_init_data(&self, init_data: Vec<usize>) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().init_data = init_data }
    }
}

#[allow(dead_code)]
impl Iterator for ObjWrap {
    type Item = ObjWrap;

    fn next(&mut self) -> Option<Self::Item> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt().ptr;
            return Some(now);
        } else {
            return None;
        }
    }
}
