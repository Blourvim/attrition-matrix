var sdks = [];

const add_sdk = (sdk_id) => {
    sdks.push(sdk_id);
}


const remove_sdk = (sdk_id) => {
    const index = sdks.indexOf(sdk_id);
    if (index > -1) {
        sdks.splice(index, 1);
    }
    return [...sdks]
}