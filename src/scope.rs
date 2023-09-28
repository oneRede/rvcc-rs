use crate::{obj::ObjWrap, ty::TyWrap, token::TokenWrap};

#[allow(dead_code)]
pub static mut SCOPE: ScopeWrap = ScopeWrap::empty();

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct VarScope {
    next: VarScopeWrap,
    name: &'static str,
    var: ObjWrap,
    typedef: TyWrap,
}

#[allow(dead_code)]
impl VarScope {
    fn new() -> Self {
        Self {
            next: VarScopeWrap::empty(),
            name: "",
            var: ObjWrap::empty(),
            typedef: TyWrap::empty(),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct VarScopeWrap {
    pub ptr: Option<*mut VarScope>,
}

#[allow(dead_code)]
impl Iterator for VarScopeWrap {
    type Item = VarScopeWrap;

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
impl VarScopeWrap {
    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new() -> Self {
        let var_scope: Option<*mut VarScope> = Some(Box::leak(Box::new(VarScope::new())));
        Self { ptr: var_scope }
    }

    pub fn nxt(&self) -> VarScopeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn var(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().var }
    }

    pub fn typedef(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().typedef }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_var(&self, var: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().var = var }
    }

    pub fn set_next(&self, next: VarScopeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_typedef(&self, typedef: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().typedef = typedef }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Scope {
    next: ScopeWrap,
    vars: VarScopeWrap,
    tags: TagScopeWrap,
}

#[allow(dead_code)]
impl Scope {
    fn new() -> Self {
        Self {
            next: ScopeWrap::empty(),
            vars: VarScopeWrap::empty(),
            tags: TagScopeWrap::empty(),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ScopeWrap {
    ptr: Option<*mut Scope>,
}

unsafe impl Sync for Scope {}

#[allow(dead_code)]
impl Iterator for ScopeWrap {
    type Item = ScopeWrap;

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
impl ScopeWrap {
    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new() -> Self {
        let scope: Option<*mut Scope> = Some(Box::leak(Box::new(Scope::new())));
        Self { ptr: scope }
    }

    pub fn enter(&self) {
        self.set_next(unsafe { SCOPE });
        unsafe { SCOPE = *self }
    }

    pub fn leave(&self) {
        unsafe { SCOPE = SCOPE.nxt() }
    }

    pub fn push(name: &'static str) -> VarScopeWrap {
        let var_scope = VarScopeWrap::new();
        var_scope.set_name(name);
        // var_scope.set_var(var);
        var_scope.set_next(unsafe { SCOPE.vars() });
        unsafe { SCOPE.set_vars(var_scope) }

        return var_scope;
    }

    pub fn nxt(&self) -> ScopeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn tags(&self) -> TagScopeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().tags }
    }

    pub fn set_next(&self, next: ScopeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_vars(&self, vars: VarScopeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().vars = vars }
    }

    pub fn vars(&self) -> VarScopeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().vars }
    }

    pub fn set_tags(&self, tags: TagScopeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().tags = tags }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TagScope {
    next: TagScopeWrap,
    name: &'static str,
    ty: TyWrap,
}

#[allow(dead_code)]
impl TagScope {
    pub fn new() -> Self {
        Self {
            next: TagScopeWrap::empty(),
            name: "",
            ty: TyWrap::empty(),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TagScopeWrap {
    ptr: Option<*mut TagScope>,
}

#[allow(dead_code)]
impl TagScopeWrap {
    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new() -> Self {
        let tag_scope: Option<*mut TagScope> = Some(Box::leak(Box::new(TagScope::new())));
        Self { ptr: tag_scope }
    }

    pub fn nxt(&self) -> TagScopeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }

    pub fn set_next(&self, next: TagScopeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn push(token: TokenWrap, ty: TyWrap) {
        let tag_scope = TagScopeWrap::new();
        let name: String = token.loc().unwrap()[..token.len()].into_iter().collect();
        let name = Box::leak(Box::new(name));
        tag_scope.set_name(name);
        tag_scope.set_ty(ty);
        tag_scope.set_next(unsafe { SCOPE.tags() });
        
        unsafe { SCOPE.set_tags(tag_scope) };
    }
}

#[allow(dead_code)]
impl Iterator for TagScopeWrap {
    type Item = TagScopeWrap;

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
pub struct VarAttr {
    pub is_typedef: Option<bool>
}

#[allow(dead_code)]
impl VarAttr {
    pub fn empty() -> Self{
        Self{
            is_typedef: Some(false),
        }
    }
}
