export class JSFileReference {
    /**
     * @param {FileSystemFileHandle} handle 
     */
    constructor(handle) {
        this.handle = handle;
    }

    /**
     * @returns {string}
     */
    name() {
        return this.handle.name;
    }

    /**
     * @returns {Uint8Array}
     */
    async readBytes() {
        try {
            const file = await this.handle.getFile();
            return new Uint8Array(await file.arrayBuffer());
        } catch (error) {
            throw transformError(error);
        }
    }

    /**
     * @param {ArrayBuffer} bytes 
     */
    async write(bytes) {
        try {
            const writable = await this.handle.createWritable();
            writable.write(bytes);
        } catch (error) {
            throw transformError(error);
        }
    }

    /**
     * @returns {number}
     */
    async modificationDate() {
        try {
            const file = await this.handle.getFile();
            return file.lastModified;
        } catch (error) {
            throw transformError(error);
        }
    }

    /**
     * @returns {number}
     */
    async size() {
        try {
            const file = await this.handle.getFile();
            return file.size;
        } catch (error) {
            throw transformError(error);
        }
    }
}

export class JSFileOrDirectoryReference {
    /**
     * @param {FileSystemHandle} handle 
     */
    constructor(handle) {
        this.handle = handle;
    }

    /**
     * @returns {string}
     */
    name() {
        return this.handle.name;
    }

    asDirectory() {
        return this.handle.kind == "directory" ? new JSDirectoryReference(this.handle) : null;
    }

    asFile() {
        return this.handle.kind == "file" ? new JSFileReference(this.handle) : null;
    }
}

export class JSDirectoryReference {
    /**
     * @param {FileSystemDirectoryHandle} handle 
     */
    constructor(handle) {
        this.handle = handle;
    }

    /**
     * @returns {string}
     */
    name() {
        return this.handle.name;
    }

    /**
     * @returns {Promise<[string, JSFileOrDirectoryReference]>}
     */
    async entries() {
        const results = [];
        for (const [name, handlePromise] of this.handle.entries()) {
            try {
                results.push(new JSFileOrDirectoryReference(await handlePromise));
            } catch (error) {
                throw transformError(error);
            }
        }
        return results;
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