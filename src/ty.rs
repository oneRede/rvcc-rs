use crate::{
    node::{NodeKind, NodeWrap},
    token::TokenWrap,
    utils::error_token,
};

#[allow(dead_code)]
pub static mut TYPE_INT: TyWrap = TyWrap::empty();

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeKind {
    INT,
    CHAR,
    PTR,
    FUNC,
    ARRAY,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TyV2 {
    pub kind: Option<TypeKind>,
    pub base: TyWrap,
    pub token: TokenWrap,
    pub return_ty: TyWrap,
    pub params: TyWrap,
    pub next: TyWrap,
    pub size: usize,
    pub array_len: usize,
}

impl TyV2 {
    pub fn new() -> Self {
        Self {
            kind: None,
            base: TyWrap::empty(),
            token: TokenWrap::empty(),
            return_ty: TyWrap::empty(),
            params: TyWrap::empty(),
            next: TyWrap::empty(),
            size: 8,
            array_len: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TyWrap {
    pub ptr: Option<*mut TyV2>,
}

#[allow(dead_code)]
impl TyWrap {
    pub fn new() -> Self {
        let ty: Option<*mut TyV2> = Some(Box::leak(Box::new(TyV2::new())));
        Self { ptr: ty }
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new_with_kind(kind: Option<TypeKind>) -> Self {
        
        let ty = TyWrap::new();
        if kind ==Some(TypeKind::INT) {
            ty.set_size(8);
        } else{
            ty.set_size(1)
        }
        ty.set_kind(kind);
        ty
    }

    pub fn new_func_ty(return_ty: TyWrap) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(Some(TypeKind::FUNC));
        ty.set_return_ty(return_ty);
        ty
    }

    pub fn new_array_ty(base: TyWrap, len: usize) -> Self {
        let ty = TyWrap::new();
        ty.set_base(base);
        ty.set_kind(Some(TypeKind::ARRAY));
        ty.set_size(ty.base().size() * len);
        ty.set_array_len(len);
        ty
    }

    pub fn point_to(base: TyWrap) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(Some(TypeKind::PTR));
        ty.set_base(base);
        ty
    }

    pub fn copy(ty: TyWrap) -> Self {
        let tmp = TyWrap::new();
        tmp.set_kind(ty.kind());
        tmp.set_base(ty.base());
        tmp.set_token(ty.token());
        tmp.set_return_ty(ty.return_ty());
        tmp.set_params(ty.params());
        tmp.set_next(ty.next());
        tmp
    }

    pub fn kind(&self) -> Option<TypeKind> {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn base(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().base }
    }

    pub fn token(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().token }
    }

    pub fn return_ty(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().return_ty }
    }

    pub fn params(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().params }
    }

    pub fn next(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn size(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().size }
    }

    pub fn array_len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().array_len }
    }

    pub fn set_kind(&self, kind: Option<TypeKind>) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind };
    }

    pub fn set_base(&self, base: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().base = base };
    }

    pub fn set_token(&self, token: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().token = token };
    }

    pub fn set_return_ty(&self, return_ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().return_ty = return_ty }
    }

    pub fn set_params(&self, params: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().params = params }
    }

    pub fn set_next(&self, next: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_size(&self, size: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().size = size }
    }

    pub fn set_array_len(&self, array_len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().array_len = array_len }
    }
}

#[allow(dead_code)]
pub fn is_int(ty: TyWrap) -> bool {
    return ty.kind() == Some(TypeKind::INT) || ty.kind() == Some(TypeKind::CHAR);
}

#[allow(dead_code)]
pub fn add_ty(node: NodeWrap) {
    if node.ptr.is_none() || !node.ty().ptr.is_none() {
        return;
    }

    add_ty(node.lhs());
    add_ty(node.rhs());
    add_ty(node.cond());
    add_ty(node.then());
    add_ty(node.els());
    add_ty(node.init());
    add_ty(node.inc());

    for nd in node.body() {
        add_ty(nd);
    }

    for nd in node.args() {
        add_ty(nd)
    }

    match node.kind() {
        NodeKind::NEG | NodeKind::Div | NodeKind::Mul | NodeKind::Sub | NodeKind::Add => {
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::ASSIGN => {
            let kind = node.lhs().ty().kind();
            if kind == Some(TypeKind::ARRAY) {
                error_token(node.lhs().token(), "not an lvalue");
            }
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::FUNCALL
        | NodeKind::EQ
        | NodeKind::NE
        | NodeKind::LT
        | NodeKind::LE
        | NodeKind::Num => {
            node.set_ty(TyWrap::new_with_kind(Some(TypeKind::INT)));
            return;
        }
        NodeKind::VAR => {
            let ty = node.var().ty();
            node.set_ty(ty);
            return;
        }
        NodeKind::ADDR => {
            let ty = node.lhs().ty();
            if ty.kind() == Some(TypeKind::ARRAY) {
                node.set_ty(TyWrap::point_to(ty.base()));
            } else {
                node.set_ty(TyWrap::point_to(ty));
            }
            return;
        }
        NodeKind::DEREF => {
            if node.lhs().ty().base().ptr.is_none() {
                error_token(node.token(), "invalid pointer dereference")
            }
            node.set_ty(node.lhs().ty().base());
            return;
        }
        _ => {}
    }
    return;
}
