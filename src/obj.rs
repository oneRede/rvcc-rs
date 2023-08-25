use crate::{parse::LOCALS, ty::TyWrap};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Obj {
    pub next: ObjWrap,
    pub name: &'static str,
    pub offset: i64,
    pub ty: TyWrap,
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
        };
        let var: Option<*mut Obj> = Some(Box::leak(Box::new(var)));
        let var = Self { ptr: var };
        var.set_nxt(unsafe { LOCALS });
        unsafe { LOCALS = var };
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

    pub fn offset(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().offset }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
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
