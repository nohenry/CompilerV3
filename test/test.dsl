
template Data<T> {
    x: T
}

template Data<T as uint32> {
    x: T
    y: bool
}

main(x: int32) => {
    let d = Data {
        x: 0,
        y: true
    } 

    let b = Data {
        x: false,
    }
}