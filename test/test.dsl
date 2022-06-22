template Data {
    x: int32
    y: int32
}

main(d: int32) => {
    let a = if d > 5 {
        4
    } else {
        d + 4
    }
}