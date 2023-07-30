use crate::{
    rvcc::{Function, Node, NodeKind, Obj, Token, TokenKind},
    tokenize::{equal, error_token, skip},
};

pub static mut LOCALS: Option<*mut Obj> = None;

#[allow(dead_code)]
pub fn expr(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    return assign(token);
}

#[allow(dead_code)]
pub fn assign(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (mut node, mut token) = equality(token);
    if equal(unsafe { token.as_ref().unwrap() }, &['=']) {
        let (n, t) = assign(unsafe { token.as_ref().unwrap().next.unwrap() });
        node = Some(Box::leak(Box::new(Node::new_binary(
            NodeKind::ASSIGN,
            node.unwrap(),
            n.unwrap(),
        ))));
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (mut node, mut token) = relational(token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['=', '=']) {
            let (n, t) = relational(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::EQ,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['!', '=']) {
            let (n, t) = relational(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::NE,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn relational(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (mut node, mut token) = add(token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['<']) {
            let (n, t) = add(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['<', '=']) {
            let (n, t) = add(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>']) {
            let (n, t) = add(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                n.unwrap(),
                node.unwrap(),
            ))));
            token = t;
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>', '=']) {
            let (n, t) = add(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                n.unwrap(),
                node.unwrap(),
            ))));
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn add(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (mut node, mut token) = mul(token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
            let (n, t) = mul(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::Add,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
            let (n, t) = mul(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::Sub,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (mut node, mut token) = unary(token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['*']) {
            let (n, t) = unary(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::Mul,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['/']) {
            let (n, t) = unary(unsafe { token.as_ref().unwrap().next.unwrap() });
            node = Some(Box::leak(Box::new(Node::new_binary(
                NodeKind::Div,
                node.unwrap(),
                n.unwrap(),
            ))));
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
        return unary(unsafe { token.as_ref().unwrap().next.unwrap() });
    }
    if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
        let (n, t) = unary(unsafe { token.as_ref().unwrap().next.unwrap() });
        return (
            Some(Box::leak(Box::new(Node::new_unary(
                NodeKind::NEG,
                n.unwrap(),
            )))),
            t,
        );
    }

    primary(token)
}

#[allow(dead_code)]
fn primary(mut token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    if equal(unsafe { token.as_ref().unwrap() }, &['(']) {
        let (n, t) = expr(unsafe { token.as_ref().unwrap().next.unwrap() });
        token = t;
        return (n, unsafe { token.as_ref().unwrap().next.unwrap() });
    }

    if (unsafe { token.as_ref().unwrap().kind } == TokenKind::IDENT) {
        let mut var = find_var(token);
        if var.is_none() {
            let len = unsafe { token.as_ref().unwrap().len };
            let name: String = unsafe { &token.as_ref().unwrap().loc.unwrap()[..len] }
                .iter()
                .collect();
            var = Some(Obj::new(Box::leak(Box::new(name))));
        }
        let node = Box::leak(Box::new(Node::new_var_node(var)));
        return (Some(node), unsafe { token.as_ref().unwrap().next.unwrap() });
    }

    if (unsafe { token.as_ref().unwrap().kind } == TokenKind::Num) {
        let node = Box::leak(Box::new(Node::new_num(
            unsafe { token.as_ref().unwrap().val } as i64,
        )));
        return (Some(node), unsafe { token.as_ref().unwrap().next.unwrap() });
    }

    error_token(unsafe { token.as_ref().unwrap() }, "expected an expression");
    (None, token)
}

#[allow(dead_code)]
fn stmt(token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    expr_stmt(token)
}

#[allow(dead_code)]
fn expr_stmt(mut token: *mut Token) -> (Option<*mut Node>, *mut Token) {
    let (n, t) = expr(token);
    let node = Box::leak(Box::new(Node::new_unary(NodeKind::ExprStmt, n.unwrap())));
    token = skip(unsafe { t.as_ref().unwrap() }, &[';']).unwrap();
    return (Some(node), token);
}

#[allow(dead_code)]
pub fn parse(mut token: *mut Token) -> *mut Function {
    let head: *mut Node = &mut Node::new(NodeKind::Num) as *mut Node;
    let mut cur = head;

    loop {
        if unsafe { token.as_ref().unwrap().kind } == TokenKind::Eof {
            break;
        }
        let (n, t) = stmt(token);
        unsafe {
            cur.as_mut().unwrap().next = n;
        }
        token = t;
        cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
    }

    let prog = Function::new(unsafe { head.as_ref().unwrap().next.unwrap() }, unsafe {
        LOCALS
    });
    return Box::leak(Box::new(prog));
}

#[allow(dead_code)]
pub fn find_var(token: *mut Token) -> Option<*mut Obj> {
    if unsafe { LOCALS.is_none() } {
        return None;
    }
    let mut var = unsafe { LOCALS.unwrap() };
    loop {
        let name: Vec<char> = unsafe { var.as_ref().unwrap().name.chars().into_iter().collect() };
        if unsafe { var.as_ref().unwrap().name.len() == token.as_ref().unwrap().len }
            && equal(unsafe { token.as_ref().unwrap() }, &name)
        {
            return Some(var);
        }
        if unsafe { var.as_ref().unwrap().next.is_none() } {
            break;
        }
        var = unsafe { var.as_ref().unwrap().next.unwrap() }
    }
    None
}
