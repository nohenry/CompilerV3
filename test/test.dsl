
template Data<T> {
    x: T
}

action <T> Data<T> {
    act(const self) => {
    }
}

main(x: int32) => {
    let data = Data {
        x: 9
    } 

    data.act()
}