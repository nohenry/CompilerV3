template Data1 {
    u: int32
    v: int32
}

template Data {
    x: int32
    d: Data1 
}

main(x: int32) => {
    let data = Data {
        x: 10,
        d: Data1 {
            u: 15,
            v: 20
        }
    }
    let g = data.d
}