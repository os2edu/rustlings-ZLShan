// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!

    // 第一种方法：直接利用trim()函数
    // input.trim().to_string()

    // 这里函数结果不满足的原因是：replace把所有空格都给删除了。这里是让前后的空格删除
    // input.to_string().replace(" ","")

    // 第二种方法：利用replacen(), pop()函数设计实现分割
    let mut s = String::from(input);
    let mut count:usize = 0;
    for a in s.chars() {  // 除去前端的空格
        if a != ' ' {
            s = s.replacen(" ","",count);
            break;
        }
        count += 1;
    }
    let len = s.len();
    let mut ss = s.clone();
    while len > 0 {  // 除去后端的空格
        if ss.pop() == Some(' ') {
            s.pop();
        } else {
            break;
        }
    }
    return s;
    
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!

    input.to_string() + " world!"

    // let mut s = String::from(input); 
    // s.push_str(" world!");
    // s

    // let mut s = input.to_string();
    // s.push_str(" world!");
    // s

    // 下边这个编译无法通过：expected struct `String`, found `()`
    // 分析：push_str  this call modifies its receiver in-place，即该方法没有返回值，需要将生成的String赋予一个变量再返回
    // input.to_string().push_str(" world!")   
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.to_string().replace("cars","balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
