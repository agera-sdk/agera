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
        const file = await handle1.getFile();
        return new Uint8Array(await file.arrayBuffer());
    } catch (error) {
        throw transformError(error);
    }
}

export async function directoryListingAsync(path) {
    const handle = await getDirectoryHandleAsync(path);
    const listing = [];
    try {
        for await (const key of handle.keys()) {
            listing.push(key);
        }
    } catch (error) {
        throw transformError(error);
    }
    return listing;
}

export async function deleteEmptyDirectoryAsync(parentPath, name) {
    const parentHandle = await getDirectoryHandleAsync(parentPath);
    try {
        await parentHandle.getDirectoryHandle(name);
        await parentHandle.removeEntry(name);
    } catch(error) {
        throw transformError(error);
    }
}

export async function deleteDirectoryAllAsync(parentPath, name) {
    const parentHandle = await getDirectoryHandleAsync(parentPath);
    try {
        await parentHandle.getDirectoryHandle(name);
        await parentHandle.removeEntry(name, { recursive: true });
    } catch(error) {
        throw transformError(error);
    }
}

export async function deleteFileAsync(parentPath, name) {
    const parentHandle = await getDirectoryHandleAsync(parentPath);
    try {
        await parentHandle.getFileHandle(name);
        await parentHandle.removeEntry(name);
    } catch(error) {
        throw transformError(error);
    }
}

export async function writeAsync(path, data) {
    const handle = await getFileHandleAsync(path, true);
    try {
        const handle2 = await handle.createWritable();
        handle2.write(data);
    } catch (error) {
        throw transformError(error);
    }
}

export async function modificationEpochMillisecondsAsync(path) {
    try {
        await getDirectoryHandleAsync(path);
        return undefined;
    } catch (error) {
        if (error != errorConstants.TypeMismatchError) {
            throw error;
        }
    }
    const handle = await getFileHandleAsync(path);
    try {
        const file = await handle.getFile();
        return file.lastModified;
    } catch (error) {
        throw transformError(error);
    }
}

export async function sizeAsync(path) {
    const handle1 = await getFileHandleAsync(path);
    try {
        const file = await handle1.getFile();
        return file.size;
    } catch (error) {
        throw transformError(error);
    }
}

/**
 * @throws {number} An error constant.
 */
async function getFileHandleAsync(path, create = false) {
    try {
        let dirHandle = await navigator.storage.getDirectory();
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
        return await dirHandle.getFileHandle(fileSegment, { create });
    } catch (e) {
        throw transformError(e);
    }
}

/**
 * @throws {number} An error constant.
 */
async function getDirectoryHandleAsync(path, create = false) {
    try {
        let handle = await navigator.storage.getDirectory();
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