use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = VecDeque::new();

    // 문자열의 각 문자를 순회
    for c in string.chars() {
        match c {
            // 여는 괄호인 경우 스택에 푸시
            '{' | '[' | '(' => stack.push_back(c),
            // 닫는 중괄호인 경우 스택의 마지막 값이 여는 중괄호인지 확인
            '}' => {
                if stack.pop_back() != Some('{') {
                    return false;
                }
            }

            // 닫는 대괄호인 경우 스택의 마지막 값이 여는 대괄호인지 확인
            ']' => {
                if stack.pop_back() != Some('[') {
                    return false;
                }
            }
            // 닫는 소괄호인 경우 스택의 마지막 값이 여는 소괄호인지 확인
            ')' => {
                if stack.pop_back() != Some('(') {
                    return false;
                }
            }
            // 괄호가 아닌 문자는 무시
            _ => continue,
        }
    }

    stack.is_empty()
}
