
template Data {
    x: int32
    y: int32
}

action Data {
    potato() => {

    }
}

main(x: int32) => {
    let data = Data {
        x: 10,
        y: 20 
    }
    data.potato()
}