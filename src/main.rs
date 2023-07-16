use std::{env, num::ParseIntError, process::exit, slice};
mod utils;

use utils::get_str_num;

#[allow(dead_code)]
#[derive(PartialEq)]
enum TokenKind {
    Punct,
    Num,
    Eof,
}

#[allow(dead_code)]
struct Token<'a> {
    kind: TokenKind,
    next: Option<*mut Token<'a>>,
    val: i32,
    loc: Option<*const str>,
    len: usize,
}

#[allow(dead_code)]
impl<'a> Token<'a> {
    fn new(token_kind: TokenKind, loc: *const u8, end: usize) -> Self {
        Self {
            kind: token_kind,
            next: None,
            val: 0,
            loc: None,
            len: 0,
        }
    }
    fn empty() -> Self {
        Self {
            kind: TokenKind::Eof,
            next: None,
            val: 0,
            loc: None,
            len: 0,
        }
    }
}

#[allow(dead_code)]
fn equal(token: &Token, s: &str) -> bool {
    if unsafe {
        std::str::from_utf8(slice::from_raw_parts(
            token.loc.unwrap() as *const u8,
            token.len,
        ))
        .unwrap()
    } == s
    {
        return true;
    } else {
        return false;
    }
}

#[allow(dead_code)]
fn skip<'a>(token: Token<'a>, s: &'a str) -> Option<*mut Token<'a>> {
    if !equal(&token, s) {
        println!("expect '{}'", s)
    }
    return token.next;
}

#[allow(dead_code)]
fn get_num(token: Token) -> i32 {
    if token.kind != TokenKind::Num {
        println!("expect a num")
    }
    token.val
}

#[allow(dead_code)]
fn tokenize(mut s: &'static str) {
    let head: *mut Token = Box::leak(Box::new(Token::empty()));
    let mut cur = head;

    loop {
        if s.len() == 0 {
            break;
        }

        let c: Vec<char> = s[..1].chars().collect();
        if c.get(0).unwrap().is_whitespace() {
            s = &s[1..];
            continue;
        }

        let num_rs: Result<i32, ParseIntError> = s.parse();
        match num_rs {
            Ok(num) => {}
            Err(e) => {
                continue;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() != &2 {
        println!("{}: invalid number of arguments\n", &args.get(0).unwrap());
        exit(1)
    }
    let mut p: &str = &args[1];

    println!("  .globl main");
    println!("main:");
    let (n, s) = get_str_num(p);
    p = s;
    println!("  li a0, {}", n);

    loop {
        if p.len() == 0 {
            break;
        }
        if p.starts_with("+") {
            let (n, s) = get_str_num(&p[1..]);
            println!("  addi a0, a0, {}", n);
            p = s;
        } else if p.starts_with("-") {
            let (n, s) = get_str_num(&p[1..]);
            println!("  addi a0, a0, -{}", n);
            p = s;
        } else {
            println!("unexcept character: {}", &p[..1]);
            exit(1)
        }
    }

    println!("  ret");
    exit(0)
}

// typedef enum {
//     TK_PUNCT, // 操作符如： + -
//     TK_NUM,   // 数字
//     TK_EOF,   // 文件终止符，即文件的最后
//   } TokenKind;

//   // 终结符结构体
//   typedef struct Token Token;
//   struct Token {
//     TokenKind Kind; // 种类
//     Token *Next;    // 指向下一终结符
//     int Val;        // 值
//     char *Loc;      // 在解析的字符串内的位置
//     int Len;        // 长度
//   };

//   // 输出错误信息
//   // static文件内可以访问的函数
//   // Fmt为传入的字符串， ... 为可变参数，表示Fmt后面所有的参数
//   static void error(char *Fmt, ...) {
//     // 定义一个va_list变量
//     va_list VA;
//     // VA获取Fmt后面的所有参数
//     va_start(VA, Fmt);
//     // vfprintf可以输出va_list类型的参数
//     vfprintf(stderr, Fmt, VA);
//     // 在结尾加上一个换行符
//     fprintf(stderr, "\n");
//     // 清除VA
//     va_end(VA);
//     // 终止程序
//     exit(1);
//   }

//   // 判断Tok的值是否等于指定值，没有用char，是为了后续拓展
//   static bool equal(Token *Tok, char *Str) {
//     // 比较字符串LHS（左部），RHS（右部）的前N位，S2的长度应大于等于N.
//     // 比较按照字典序，LHS<RHS回负值，LHS=RHS返回0，LHS>RHS返回正值
//     // 同时确保，此处的Op位数=N
//     return memcmp(Tok->Loc, Str, Tok->Len) == 0 && Str[Tok->Len] == '\0';
//   }

//   // 跳过指定的Str
//   static Token *skip(Token *Tok, char *Str) {
//     if (!equal(Tok, Str))
//       error("expect '%s'", Str);
//     return Tok->Next;
//   }

//   // 返回TK_NUM的值
//   static int getNumber(Token *Tok) {
//     if (Tok->Kind != TK_NUM)
//       error("expect a number");
//     return Tok->Val;
//   }

//   // 生成新的Token
//   static Token *newToken(TokenKind Kind, char *Start, char *End) {
//     // 分配1个Token的内存空间
//     Token *Tok = calloc(1, sizeof(Token));
//     Tok->Kind = Kind;
//     Tok->Loc = Start;
//     Tok->Len = End - Start;
//     return Tok;
//   }

//   // 终结符解析
//   static Token *tokenize(char *P) {
//     Token Head = {};
//     Token *Cur = &Head;

//     while (*P) {
//       // 跳过所有空白符如：空格、回车
//       if (isspace(*P)) {
//         ++P;
//         continue;
//       }

//       // 解析数字
//       if (isdigit(*P)) {
//         // 初始化，类似于C++的构造函数
//         // 我们不使用Head来存储信息，仅用来表示链表入口，这样每次都是存储在Cur->Next
//         // 否则下述操作将使第一个Token的地址不在Head中。
//         Cur->Next = newToken(TK_NUM, P, P);
//         // 指针前进
//         Cur = Cur->Next;
//         const char *OldPtr = P;
//         Cur->Val = strtoul(P, &P, 10);
//         Cur->Len = P - OldPtr;
//         continue;
//       }

//       // 解析操作符
//       if (*P == '+' || *P == '-') {
//         // 操作符长度都为1
//         Cur->Next = newToken(TK_PUNCT, P, P + 1);
//         Cur = Cur->Next;
//         ++P;
//         continue;
//       }

//       // 处理无法识别的字符
//       error("invalid token: %c", *P);
//     }

//     // 解析结束，增加一个EOF，表示终止符。
//     Cur->Next = newToken(TK_EOF, P, P);
//     // Head无内容，所以直接返回Next
//     return Head.Next;
//   }
