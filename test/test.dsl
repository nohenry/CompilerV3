
value<T, U>(a: T, b: U): T => {
    a
}

value<T as bool, U>(a: T, b: U): T => {
    if a == true {
        false
    } else {
        true 
    }
}

main() => {
    value(true, 4)
    value(false, 0)
}