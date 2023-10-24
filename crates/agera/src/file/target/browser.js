const errorConstants = {
    "NotFoundError": 0,
    "TypeMismatchError": 1,
    "NotAllowedError": 2,
    // Error thrown when a segment contains invalid characters.
    "TypeError": 3,
    "InvalidStateError": 4,
    "NoModificationAllowedError": 5,
};

export async function existsAsync(path) {
    try {
        await getFileHandleAsync(path);
        return true;
    } catch (error) {
        try {
            await getDirectoryHandleAsync(path);
            return true;
        } catch (error) {
            return false;
        }
    }
}

export async function isDirectoryAsync(path) {
    try {
        await getFileHandleAsync(path);
        return false;
    } catch (error) {
        try {
            await getDirectoryHandleAsync(path);
            return true;
        } catch (error) {
            return false;
        }
    }
}

export async function isFileAsync(path) {
    try {
        await getFileHandleAsync(path);
        return true;
    } catch (error) {
        try {
            await getDirectoryHandleAsync(path);
            return false;
        } catch (error) {
            return false;
        }
    }
}

export async function createDirectoryAsync(parentPath, name) {
    const directory = await getDirectoryHandleAsync(parentPath);
    try {
        await directory.getDirectoryHandle(name, { create: true });
    } catch (error) {
        throw transformError(e);
    }
}

export async function createDirectoryAllAsync(path) {
    await getDirectoryHandleAsync(path, true);
}

export async function readBytesAsync(path) {
    const handle1 = await getFileHandleAsync(path);
    try {
        const handle2 = await handle1.getSyncAccessHandle();
        const data = new Uint8Array(handle2.getSize());
        handle2.read(data);
        handle2.close();
        return data;
    } catch (error) {
        throw transformError(error);
    }
}

/**
 * @throws {number} An error constant.
 */
async function getFileHandleAsync(path) {
    try {
        let dirHandle = navigator.storage.getDirectory().await;
        const dirSegments = path.split('/');
        const fileSegment = dirSegments.pop();
        for (const segment of dirSegments) {
            if (segment.length === 0) {
                continue;
            }
            try {
                dirHandle = await dirHandle.getDirectoryHandle(segment);
            } catch (error) {
                if (["TypeMismatchError", "TypeError"].includes(error.name)) {
                    throw errorConstants.NotFoundError;
                } else {
                    throw error;
                }
            }
        }
        return await dirHandle.getFileHandle(fileSegment);
    } catch (e) {
        throw transformError(e);
    }
}

/**
 * @throws {number} An error constant.
 */
async function getDirectoryHandleAsync(path, create = false) {
    try {
        let handle = navigator.storage.getDirectory().await;
        const segments = path.split('/');
        for (const segment of segments) {
            if (segment.length === 0) {
                continue;
            }
            handle = await handle.getDirectoryHandle(segment, { create });
        }
        return handle;
    } catch (e) {
        throw transformError(e);
    }
}

function transformError(error) {
    if (typeof error === "number") {
        return error;
    }
    return errorConstants[error.name];
}