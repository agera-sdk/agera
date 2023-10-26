export class JSFileSystemReference {
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
     * @returns {Promise<[string, JSFileSystemReference]>}
     */
    async entries() {
        const results = [];
        for (const [name, handlePromise] of this.handle.entries()) {
            try {
                results.push(new JSFileSystemReference(await handlePromise));
            } catch (error) {
                throw transformError(error);
            }
        }
        return results;
    }

    async getDirectory(name, create) {
        try {
            return new JSDirectoryReference(await this.handle.getDirectoryHandle(name, { create }));
        } catch (error) {
            throw transformError(error);
        }
    }

    async getFile(name, create) {
        try {
            return new JSFileReference(await this.handle.getFileHandle(name, { create }));
        } catch (error) {
            throw transformError(error);
        }
    }

    async deleteEmptyDirectory(name) {
        try {
            await this.handle.getDirectoryHandle(name);
            await this.handle.removeEntry(name);
        } catch (error) {
            throw transformError(error);
        }
    }

    async deleteDirectoryAll(name) {
        try {
            await this.handle.getDirectoryHandle(name);
            await this.handle.removeEntry(name, { recursive: true });
        } catch (error) {
            throw transformError(error);
        }
    }

    async deleteFile(name) {
        try {
            await this.handle.getFileHandle(name);
            await this.handle.removeEntry(name);
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