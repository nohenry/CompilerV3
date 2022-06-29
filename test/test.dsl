
template Data<T> {
    x: T
}

action Data<uint32> {
    act(const self) => {
    }
}

main(x: int32) => {
    const data = Data {
        x: 9
    } 

}