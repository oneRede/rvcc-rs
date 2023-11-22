use crate::{
    node::{NodeWrap, MemberWrap},
    parse::{GLOBALS, LOCALS},
    scope::ScopeWrap,
    token::TokenWrap,
    ty::{TyWrap, TypeKind},
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
    pub is_static: bool,

    pub body: NodeWrap,
    pub locals: ObjWrap,
    pub stack_size: usize,
    pub params: ObjWrap,
    pub init_data: Vec<usize>,
}

#[allow(dead_code)]
impl Obj {
    pub fn new() -> Self {
        Obj {
            next: ObjWrap::empty(),
            name: "",
            offset: 0,
            ty: TyWrap::empty(),
            is_local: true,
            is_function: false,
            is_definition: false,
            is_static: false,
            body: NodeWrap::empty(),
            locals: ObjWrap::empty(),
            stack_size: 0,
            params: ObjWrap::empty(),
            init_data: vec![],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ObjWrap {
    pub ptr: Option<*mut Obj>,
}

#[allow(dead_code)]
impl ObjWrap {
    pub fn new(name: &'static str, ty: TyWrap) -> Self {
        let var = Obj::new();
        let var: Option<*mut Obj> = Some(Box::leak(Box::new(var)));
        let var = Self { ptr: var };
        var.set_name(name);
        var.set_ty(ty);

        let scope = ScopeWrap::push(name);
        scope.set_var(var);
        return var;
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

    pub fn is_static(&self) -> bool {
        unsafe { self.ptr.unwrap().as_ref().unwrap().is_static }
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

    pub fn set_is_static(&self, is_static: bool) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().is_static = is_static }
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Initializer {
    pub next: InitializerWrap,
    pub ty: TyWrap,
    pub token: TokenWrap,
    pub expr: NodeWrap,
    pub child: Vec<InitializerWrap>,
    pub is_flexible: bool,
}

#[allow(dead_code)]
impl Initializer {
    pub fn new() -> Self {
        Self {
            next: InitializerWrap::empty(),
            ty: TyWrap::empty(),
            token: TokenWrap::empty(),
            expr: NodeWrap::empty(),
            child: vec![],
            is_flexible: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct InitializerWrap {
    pub ptr: Option<*mut Initializer>,
}

#[allow(dead_code)]
impl InitializerWrap {
    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new(ty: TyWrap, is_flexible: bool) -> Self {
        let init = Initializer::new();
        let init: *mut Initializer = Box::leak(Box::new(init));
        let init = Self { ptr: Some(init) };
        init.set_ty(ty);

        if ty.kind() == Some(TypeKind::ARRAY) {
            if is_flexible && ty.size() < 0 {
                init.set_is_flexible(true);
                return init;
            }
            for _ in 0..ty.array_len() {
                let child = InitializerWrap::new(ty.base(), false);
                init.append(child);
            }
        }

        if ty.kind() == Some(TypeKind::STRUCT) || ty.kind() == Some(TypeKind::UNION){
            for mem in ty.mems() {
                let child = InitializerWrap::new(mem.ty(), false);
                init.append(child);
            }
        }
        init
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn child(&self) -> &Vec<InitializerWrap> {
        unsafe { &self.ptr.unwrap().as_ref().unwrap().child }
    }

    pub fn mut_child(&self) -> &mut Vec<InitializerWrap> {
        unsafe { &mut self.ptr.unwrap().as_mut().unwrap().child }
    }

    pub fn expr(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().expr }
    }

    pub fn is_flexible(&self) -> bool {
        unsafe { self.ptr.unwrap().as_ref().unwrap().is_flexible }
    }

    pub fn set_child(&self, child: Vec<InitializerWrap>) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().child = child }
    }

    pub fn append(&self, child: InitializerWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().child.push(child) }
    }

    pub fn set_expr(&self, expr: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().expr = expr }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }

    pub fn set_is_flexible(&self, is_flexible: bool) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().is_flexible = is_flexible }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InitDesig {
    pub next: InitDesigWrap,
    pub idx: i64,
    pub var: ObjWrap,
    pub mem: MemberWrap,
}

#[allow(dead_code)]
impl InitDesig {
    pub fn new() -> Self {
        Self {
            next: InitDesigWrap::empty(),
            idx: 0,
            var: ObjWrap::empty(),
            mem: MemberWrap::empty(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InitDesigWrap {
    pub ptr: Option<*mut InitDesig>,
}

#[allow(dead_code)]
impl InitDesigWrap {
    pub fn new() -> Self {
        let design: *mut InitDesig = Box::leak(Box::new(InitDesig::new()));
        Self { ptr: Some(design) }
    }

    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn var(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().var }
    }

    pub fn next(&self) -> &InitDesigWrap {
        unsafe { &self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn idx(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().idx }
    }

    pub fn mem(&self) -> MemberWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().mem }
    }

    pub fn set_next(&self, next: InitDesigWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_idx(&self, idx: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().idx = idx }
    }

    pub fn set_var(&self, var: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().var = var }
    }

    pub fn set_mem(&self, mem: MemberWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().mem = mem }
    }
}
