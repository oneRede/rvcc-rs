use crate::{
    node::{MemberWrap, NodeKind, NodeWrap},
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
    STR,
    STRUCT,
    UNION,
    LONG,
    SHORT,
    VOID,
    BOOL,
    ENUM,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ty {
    pub kind: Option<TypeKind>,
    pub base: TyWrap,
    pub name: TokenWrap,
    pub return_ty: TyWrap,
    pub params: TyWrap,
    pub next: TyWrap,
    pub size: usize,
    pub array_len: usize,
    pub mems: MemberWrap,
    pub align: usize,
}

impl Ty {
    pub fn new() -> Self {
        Self {
            kind: None,
            base: TyWrap::empty(),
            name: TokenWrap::empty(),
            return_ty: TyWrap::empty(),
            params: TyWrap::empty(),
            next: TyWrap::empty(),
            size: 8,
            array_len: 0,
            mems: MemberWrap::empty(),
            align: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TyWrap {
    pub ptr: Option<*mut Ty>,
}

#[allow(dead_code)]
impl TyWrap {
    pub fn new() -> Self {
        let ty: Option<*mut Ty> = Some(Box::leak(Box::new(Ty::new())));
        Self { ptr: ty }
    }

    pub fn new_v2(kind: TypeKind, size: usize, align: usize) -> Self {
        let ty = Self::new();
        ty.set_kind(kind);
        ty.set_size(size);
        ty.set_align(align);
        ty
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new_with_kind(kind: TypeKind) -> Self {
        let ty = TyWrap::new();
        if kind == TypeKind::VOID {
            ty.set_size(1);
            ty.set_align(1);
        } else if kind == TypeKind::BOOL {
            ty.set_size(1);
            ty.set_align(1);
        } else if kind == TypeKind::CHAR {
            ty.set_size(1);
            ty.set_align(1);
        } else if kind == TypeKind::SHORT {
            ty.set_size(2);
            ty.set_align(2);
        } else if kind == TypeKind::ENUM {
            ty.set_size(4);
            ty.set_align(4);
        } else if kind == TypeKind::INT {
            ty.set_size(4);
            ty.set_align(4);
        } else {
            ty.set_size(8);
            ty.set_align(8);
        }
        ty.set_kind(kind);
        ty
    }

    pub fn new_func_ty(return_ty: TyWrap) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(TypeKind::FUNC);
        ty.set_return_ty(return_ty);
        ty
    }

    pub fn new_array_ty(base: TyWrap, len: usize) -> Self {
        let ty = TyWrap::new_v2(TypeKind::ARRAY, base.size() * len, base.align());
        ty.set_base(base);
        ty.set_array_len(len);
        ty
    }

    pub fn point_to(base: TyWrap) -> Self {
        let ty = TyWrap::new_v2(TypeKind::PTR, 8, 8);
        ty.set_base(base);
        ty
    }

    pub fn kind(&self) -> Option<TypeKind> {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn base(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().base }
    }

    pub fn name(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
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

    pub fn mems(&self) -> MemberWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().mems }
    }

    pub fn align(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().align }
    }

    pub fn set_kind(&self, kind: TypeKind) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = Some(kind) };
    }

    pub fn set_base(&self, base: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().base = base };
    }

    pub fn set_token(&self, name: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name };
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

    pub fn set_mems(&self, mems: MemberWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().mems = mems }
    }

    pub fn set_align(&self, align: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().align = align }
    }
}

#[allow(dead_code)]
pub fn is_int(ty: TyWrap) -> bool {
    return ty.kind() == Some(TypeKind::ENUM)
        || ty.kind() == Some(TypeKind::BOOL)
        || ty.kind() == Some(TypeKind::INT)
        || ty.kind() == Some(TypeKind::CHAR)
        || ty.kind() == Some(TypeKind::LONG)
        || ty.kind() == Some(TypeKind::SHORT);
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
        NodeKind::Num => {
            let ty: TyWrap = if node.val() == node.val() as u32 as i64 {
                TyWrap::new_with_kind(TypeKind::INT)
            } else {
                TyWrap::new_with_kind(TypeKind::LONG)
            };
            node.set_ty(ty)
        }
        NodeKind::Div | NodeKind::Mul | NodeKind::Sub | NodeKind::Add => {
            let (lhs, rhs) = usual_arith_conv(node.lhs(), node.rhs());
            node.set_lhs(lhs);
            node.set_rhs(rhs);
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::NEG => {
            let ty = get_common_ty(TyWrap::new_with_kind(TypeKind::INT), node.lhs().ty());
            node.set_lhs(NodeWrap::new_cast(node.lhs(), ty));
            node.set_ty(ty);
            return;
        }
        NodeKind::ASSIGN => {
            let kind = node.lhs().ty().kind();
            if kind == Some(TypeKind::ARRAY) {
                error_token(node.lhs().token(), "not an lvalue");
            }
            if node.lhs().ty().kind() != Some(TypeKind::STRUCT) {
                node.set_rhs(NodeWrap::new_cast(node.rhs(), node.lhs().ty()));
            }
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::EQ | NodeKind::NE | NodeKind::LT | NodeKind::LE => {
            let (lhs, rhs) = usual_arith_conv(node.lhs(), node.rhs());
            node.set_lhs(lhs);
            node.set_rhs(rhs);
            node.set_ty(TyWrap::new_with_kind(TypeKind::INT));
            return;
        }
        NodeKind::FUNCALL => {
            node.set_ty(node.func_type().return_ty());
            return;
        }
        NodeKind::NOT => {
            node.set_ty(TyWrap::new_with_kind(TypeKind::INT));
            return;
        }
        NodeKind::VAR => {
            let ty = node.var().ty();
            node.set_ty(ty);
            return;
        }
        NodeKind::COMMA => {
            node.set_ty(node.rhs().ty());
            return;
        }
        NodeKind::MEMBER => {
            node.set_ty(node.mem().ty());
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
            if node.lhs().ty().base().kind() == Some(TypeKind::VOID) {
                error_token(node.token(), "dereferencing a void pointer")
            }
            node.set_ty(node.lhs().ty().base());
            return;
        }

        NodeKind::STMTEXPR => {
            if !node.body().ptr.is_none() {
                let mut stmt = node.body();
                while !stmt.nxt().ptr.is_none() {
                    stmt = stmt.nxt();
                }
                if stmt.kind() == NodeKind::EXPRSTMT {
                    node.set_ty(stmt.lhs().ty());
                    return;
                }
            }
            error_token(
                node.token(),
                "statement expression returning void is not supported",
            );
            return;
        }
        _ => {}
    }
    return;
}

#[allow(dead_code)]
pub fn get_common_ty(ty1: TyWrap, ty2: TyWrap) -> TyWrap {
    if !ty1.base().ptr.is_none() {
        return TyWrap::point_to(ty1.base());
    }
    if ty1.size() == 8 || ty2.size() == 8 {
        return TyWrap::new_with_kind(TypeKind::LONG);
    }

    return TyWrap::new_with_kind(TypeKind::INT);
}

#[allow(dead_code)]
pub fn usual_arith_conv(lhs: NodeWrap, rhs: NodeWrap) -> (NodeWrap, NodeWrap) {
    let ty = get_common_ty(lhs.ty(), rhs.ty());

    let lhs = NodeWrap::new_cast(lhs, ty);
    let rhs = NodeWrap::new_cast(rhs, ty);

    return (lhs, rhs);
}
