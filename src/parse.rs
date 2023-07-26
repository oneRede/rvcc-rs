use crate::{
    rvcc::{Node, NodeKind, Token, TokenKind},
    tokenize::{equal, error_token, skip},
};

#[allow(dead_code)]
pub fn expr(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    return equality(_rest, token);
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
fn stmt(mut _rest: *mut *mut Token, token: *mut Token)  -> *mut Node{
    expr_stmt(_rest, token)
}

#[allow(dead_code)]
fn expr_stmt(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node{
    let node = Box::leak(Box::new(Node::new_unary(NodeKind::ExprStmt, expr(&mut token,token))));
    unsafe { *_rest = skip(token.as_ref().unwrap(), &[';']).unwrap() };
    return node;
}

#[allow(dead_code)]
pub fn parse(mut token: *mut Token) -> *mut Node {
    let head: *mut Node = &mut Node::new(NodeKind::Num) as *mut Node;
    let mut cur = head;

    loop{
        if unsafe { token.as_ref().unwrap().kind } == TokenKind::Eof {
            break;
        }
        unsafe { cur.as_mut().unwrap().next = Some(stmt(&mut token, token)) };
        cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
    }
    return unsafe { head.as_ref().unwrap().next.unwrap() };
    
}
