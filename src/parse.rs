use crate::{
    rvcc::{Node, NodeKind, Obj, Token, TokenKind, Function},
    tokenize::{equal, error_token, skip},
};

pub static mut LOCALS: Option<*mut Obj> = None;

#[allow(dead_code)]
pub fn expr(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    return assign(_rest, token);
}

#[allow(dead_code)]
pub fn assign(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = equality(&mut token, token);

    if equal(unsafe { token.as_ref().unwrap() }, &['=']) {
        node = Box::leak(Box::new(Node::new_binary(
            NodeKind::ASSIGN,
            node,
            assign(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
        )));
    }

    unsafe { *_rest = token };
    return node;
}

#[allow(dead_code)]
fn equality(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node: *mut Node = relational(&mut token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['=', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::EQ,
                node,
                relational(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['!', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::NE,
                node,
                relational(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn relational(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = add(&mut token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['<']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                node,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['<', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                node,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
                node,
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
                node,
            )));
            continue;
        }

        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn add(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = mul(&mut token as *mut *mut Token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Add,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Sub,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn mul(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = unary(&mut token as *mut *mut Token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['*']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Mul,
                node,
                unary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['/']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Div,
                node,
                unary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn unary(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
        return unary(_rest, unsafe { token.as_ref().unwrap().next.unwrap() });
    }
    if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
        return Box::leak(Box::new(Node::new_unary(
            NodeKind::NEG,
            unary(_rest, unsafe { token.as_ref().unwrap().next.unwrap() }),
        )));
    }
    primary(_rest, token).unwrap()
}

#[allow(dead_code)]
fn primary(mut _rest: *mut *mut Token, mut token: *mut Token) -> Option<*mut Node> {
    if equal(unsafe { token.as_ref().unwrap() }, &['(']) {
        let node = expr(
            &mut token as *mut *mut Token,
            unsafe { token.as_ref().unwrap().next }.unwrap(),
        );
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap() };
        return Some(node);
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
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap() };
        return Some(node);
    }

    if (unsafe { token.as_ref().unwrap().kind } == TokenKind::Num) {
        let node = Box::leak(Box::new(Node::new_num(
            unsafe { token.as_ref().unwrap().val } as i64,
        )));
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap() };
        return Some(node);
    }

    error_token(unsafe { token.as_ref().unwrap() }, "expected an expression");
    None
}

#[allow(dead_code)]
fn stmt(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    expr_stmt(_rest, token)
}

#[allow(dead_code)]
fn expr_stmt(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let node = Box::leak(Box::new(Node::new_unary(
        NodeKind::ExprStmt,
        expr(&mut token, token),
    )));
    unsafe { *_rest = skip(token.as_ref().unwrap(), &[';']).unwrap() };
    return node;
}

#[allow(dead_code)]
pub fn parse(mut token: *mut Token) -> *mut Function {
    let head: *mut Node = &mut Node::new(NodeKind::Num) as *mut Node;
    let mut cur = head;

    loop {
        if unsafe { token.as_ref().unwrap().kind } == TokenKind::Eof {
            break;
        }
        unsafe { cur.as_mut().unwrap().next = Some(stmt(&mut token, token)) };
        cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
    }

    let prog = Function::new(unsafe { head.as_ref().unwrap().next.unwrap() }, unsafe { LOCALS });
    return Box::leak(Box::new(prog))
}

#[allow(dead_code)]
pub fn find_var(token: *mut Token) -> Option<*mut Obj> {
    if unsafe { LOCALS.is_none() }{
        return None;
    }
    let mut var = unsafe { LOCALS.unwrap()};
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
