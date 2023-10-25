const errorConstants = {
    "NotFoundError": 0,
    "TypeMismatchError": 1,
    "NotAllowedError": 2,
    // Error thrown when a segment contains invalid characters.
    "TypeError": 3,
    "InvalidStateError": 4,
    "NoModificationAllowedError": 5,
    "InvalidModificationError": 6,
};

function transformError(error) {
    if (typeof error === "number") {
        return error;
    }
    return errorConstants[error.name];
}