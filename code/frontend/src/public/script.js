let sdks = [33, 125, 25];

const add_sdk = (sdk_id) => {
    sdks.push(sdk_id);
    update_tags()
}


const remove_sdk = (sdk_id) => {
    sdks.remove(sdk_id);
    update_tags()
}

