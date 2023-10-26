export class JSFileReference {
    /**
     * @param {FileSystemFileHandle} handle 
     */
    constructor(handle) {
        this.handle = handle;
    }

    /**
     * @param {JSFileReference} other
     */
    equals(other) {
        return this.handle.isSameEntry(other.handle);
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
     * @returns {string}
     */
    async name() {
        try {
            const file = await this.handle.getFile();
            return file.name;
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