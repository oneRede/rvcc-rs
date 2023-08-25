use crate::{node::NodeWrap, obj::ObjWrap};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub next: FunctionWrap,
    pub name: &'static str,
    pub body: NodeWrap,
    pub locals: ObjWrap,
    pub stack_size: i64,
    pub params: ObjWrap,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct FunctionWrap {
    pub ptr: Option<*mut Function>,
}

#[allow(dead_code)]
impl Iterator for FunctionWrap {
    type Item = FunctionWrap;

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

#[allow(dead_code)]
impl FunctionWrap {
    pub fn new(body: NodeWrap, locals: ObjWrap) -> Self {
        let func = Function {
            next: FunctionWrap::empty(),
            name: "",
            body: body,
            locals: locals,
            stack_size: 0,
            params: ObjWrap::empty(),
        };
        let func: Option<*mut Function> = Some(Box::leak(Box::new(func)));
        Self { ptr: func }
    }

    pub fn init() -> Self {
        let func = Function {
            next: FunctionWrap::empty(),
            name: "",
            body: NodeWrap::empty(),
            locals: ObjWrap::empty(),
            stack_size: 0,
            params: ObjWrap::empty(),
        };
        let func: Option<*mut Function> = Some(Box::leak(Box::new(func)));
        Self { ptr: func }
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn nxt(&self) -> FunctionWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn body(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().body }
    }

    pub fn locals(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().locals }
    }

    pub fn stack_size(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().stack_size }
    }

    pub fn params(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().params }
    }

    pub fn set_nxt(&self, next: FunctionWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_body(&self, body: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().body = body }
    }

    pub fn set_locals(&self, locals: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().locals = locals }
    }

    pub fn set_stack_size(&self, stack_size: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().stack_size = stack_size }
    }

    pub fn set_params(&self, params: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().params = params }
    }
}
