
template Data {
    x: uint32
}

action uint32 {
    inc(self) => {

    }
}

action Data {
    act(const self) => {
    }
}

main(x: int32) => {
    const data = Data {
        x: 9
    } 
    data.act()

    (8).inc()
}